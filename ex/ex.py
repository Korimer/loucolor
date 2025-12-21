import traceback
import pandas as pd

from pyspark.sql import SparkSession
from pyspark.sql import functions as F

from data_aggregation import risk_testing
from data_aggregation.joining.CompleteDataset import CompleteDataset
from data_aggregation.risk_testing import *
from machine_learning.clustering import *
from machine_learning.features import *
from machine_learning.models import *
from visual.plot import *

def main():
    try:
        spark = SparkSession.builder.config("spark.driver.memory", "15g").getOrCreate()
        df_dict = CompleteDataset(spark, all_states())
        df = df_dict.all
        df = (df
            .withColumn(
                "distancesToHospital",
                F.transform(
                    "hospitals",
                    lambda arr: risk_testing.compute_haversine(F.col("cityLat"),F.col("cityLng"),arr.lat,arr.lng)
                )
            )
            .withColumn(
                "minDistance",
                F.array_min("distancesToHospital")
            )
        )
        risk_metric = risk_testing.create_metric(df)
        df = df.join(risk_metric, "uniqueId")
        spark.sparkContext.setLogLevel("ERROR")

        df = (df
            .withColumn("hospital",F.explode("hospitals"))
            .select("*","hospital.*")
            .drop("hospital")
        )

        # these lists need to be updated if addtional features are selected that need to be processed
        multi_categorical_feats = ["trauma", "type", "owner"]
        binary_categorical_feats = ["helipad", "status"]
        numerical_feats = ["beds", "SVISocioeconomic", "SVITotal", "SVIHousehold",
                           "SVIMinority", "SVITransportation"]

        pipeline = build_transformer_pipeline(
            multi_categorical_feats, 
            binary_categorical_feats, 
            numerical_feats
        )
        df = pipeline.fit(df).transform(df)

        df, model = run_decision_tree(df)
        df, kmeans_model = run_kmeans(df, 50)
        CENTER_COUNT = 50
        centers, topN = get_centers_and_municipalities(df, kmeans_model, CENTER_COUNT)
        predicted_min_max = df.agg(F.min("prediction"), F.max("prediction")).collect()
        predicted_min = predicted_min_max[0][0]
        predicted_max = predicted_min_max[0][1]
        print("Decision tree RMSE: ", model.avgMetrics[0] / (predicted_max - predicted_min))

        topN = topN.select("dist", "state", "city", "cityLat", "cityLng", "centroidId").distinct()

        centers = kmeans_model.clusterCenters()
        centroid_rows = [    
            (i, float(c[0]), float(c[1]))
            for i, c in enumerate(centers)
        ]
        centroids_df = spark.createDataFrame(
            centroid_rows,
            ["centroidId", "centroidLat", "centroidLng"]
        )

        final_df = topN.join(centroids_df, "centroidId").cache()
        final_df.show(n=CENTER_COUNT, truncate=False)

        cities = final_df.select("city", "cityLat", "cityLng").collect()
        generatehtml(centers, cities)
    except Exception:
        traceback.print_exc()

def all_states():
    return [
            "KY","IA","VA","OR","WY","SC","CT","FL","GA","TN",
            "AK","KS","MT","WV","ME","ND","MD","NJ","AR","NH",
            "WI","AL","LA","OK","TX","UT","CO","NM","SD","NY",
            "PA","ID","AZ","IN","MA","RI","MI","NV","MS","MN",
            "CA","NE","OH","DE","NC","VT","MO","IL", "HI","WA"
            ]

if __name__ == "__main__":
    main()
