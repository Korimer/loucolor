//package examples;
import java.util.ArrayList;
import java.util.regex.Pattern;
public class Ex {

  public static void main(String[] args) {
    Integer[] ints = new Integer[] {
      1,2,3,4,5
    };

    ListOfManyInts l = new ListOfManyInts(6,7,6,7,6,7);
    System.out.println(l.arr.get(1));

    Pattern pat = Pattern.compile("\\nthis is \\d+ regex.");
  }
}

class ListOfManyInts {
  private boolean unusedvalue;
  public ArrayList<Integer> arr = new ArrayList<>();
  public ListOfManyInts(int... manyints) {
    for (int i : manyints) {
      this.arr.add(i);
    }
  }
}
