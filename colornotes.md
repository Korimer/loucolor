# TEXT
## Variables
- Variable
- - What I'll be seeing most of. Commonly a light blue or pure white. If you make it white I'll punch a wall
- - You might want to consider it the "default" color - a majority of this text will be this.
- Builtin Variable
- - A variable intentionallly made distinct from others. Commonly a dark blue.
- - Its main purpose is to be distinct from standard variables, so it should have a color that makes it stand out from them (while still being recognizable as a variable)
- Parameter Variable
- - An in-between of a standard and a builtin variable. Should either appear very frequently or very infrequently - code that sees many parameters next to many variables is often eyebrow raising.
- - Commonly a light blue (same as standard variables), but I strongly disagree with that mindset. Considering the point above, I actually appreciate it when this color somewhat clashes with standard variables.
- Member Variable / Property
- - *Always* right next to a variable of some sort. Whether that's a builtin, parameter, or standard is uncertain.
- - Commonly a single color, rather than a unique shade depending on the type of the variable that preceeds it.
- - I prefer it as a single color way, but if making it a single color would necessetate making standard and parameter veriables too similar, you can totally make this 2 or 3 different colors.
- Constant
- - Like a variable, but with an unchanging value - making it more like an alias for something predetermined.
- - Generally stylized IN\_ALL\_CAPS so they already stand out. Thus, they don't need to be a color that stands out incredibly hard - just not the exact same color as others.

## Declarations
- Import
- - Always at the top of a file, like a header - and rarely needs to be looked at long/closely. A lot of colorschemes make it a brighter, more contrasting/jarring color for this reason.
- Namespace
- - Will always show up by an import, but very often will *also* show up across the whole file.
- - Doesn't demand high attention, but it *is* very important to distinguish it from variables. Less so to distinguish it from other text.
- Type / Modifier
- - Technically seperate (there are many modifiers for every one type), but serve the same effective purpose. Can be similar shades, the same color, etc.
- - Similar to constants, organized code will make it obvious via indentation/structure what these are. Thus, the only priority is making them distinct rather than eye-catchy.
- - These are also fairly scarce - serving effectively as sub-headers to one's code - so you can get away with a collor that's better as a highlight than as a base.
- Function
- - in a very similar boat to types/modifiers, but slightly more common. Can be considered sub-subheaders, lol.
- - commonly shades of purple or a faint reddish orange.
- Macro
- - An alternate type of function with behavior that breaks out of its general scope. These should look a little bit off-color, because they *should* look out of place next to standard functions, variables, etc.
- - Shades of purple or magenta are really popular for macros. Just please ensure it's distinct (even without a direct side-by-side) from functions.
- Repeat
- - Ok you're gonna strangle me here but these are the sub-sub-subheaders. Just like all the other header-esque items, they generally stand out by merit of the layout of the code alone. However, they are fairly common, so a faded color probably wouldn't look very good.
- - It's actually fairly common to make this the same color as Functions! If you're using a unique or standout color for functions, I'd love that! If you're using a more mundane color for functions, I'd ask for functions/repeats to be somewhat distinct (eg, different shades)
- Conditional
- - Strucutrally identical to a Repeat. Commonly the same color as one, too.
- - I honestly think you can get away with just about anything for this - faded colors work, bright colors work, the same colors as a repeat/function works, different colors from each works too. I guess if you need a color to "balance" the overall colorscheme, this might be a good spot to put it?

## Formalities
- Return
- - Phrases that terminate a logical statement. Very commonly a similar color to function text.
- - Should stand out to a decent degree - though they conclude statements, they don't *always* occur at the end of statements. Eg, in "if (thereIsAnError): return; else ...", `return` should stand out.
- Exception
- - Statements relating to error handling and unexpected behavior. Very common, but very rarely demanding of attention - so darker or more muted colors are appropriate. Commonly a dark purple (though some colorschemes are deranged and do a bright pink)
- - It's slightly uncommon, but some colorschemes make these the same color as Conditionals. This only really works if conditionals don't share a color with functions. 
- Type
- - These are really tricky - depending on the language, they can show up basically anywhere, and they could be super sparse or super abundant. Thus, they need to stand out while *also* avoiding clashing with anything noted above. Most commonly, this means making it a far brighter or far darker shade of whatever color you're using for variables.
- Decorator
- - A specification/compliment to a Type. Generally optional, but very important to functionality when they are included - so making them faded or subtle isn't on the table.
- - Commonly the opposite of a type: if a type is a bight shade of a color, a decorator is a dark shade of the same color - and vice versa

## Literals
- Integers
- - These should probably stand out. It's not like I'll have my entire screen covered by numbers, so having them be a more high contrast color is perfectly fine.
- Floats
- - Non-integer numbers (eg. 1.5 instead of 1 or 2). This can certainly be the same color as integers - though if possible, having a slightly darker shade for them would be nice- Boolean
- - The text TRUE, True, true, FALSE, False, or false. (:adachi:)
- - These are self-explanatory and obvious, so as long as they're a far cry from whatever standard variables are colored, you're safe.
- - I've commonly seen them colored either similarly to Constants, or with very dark shades. Either works for me. (I dont think many items here work well with darker shades, so this might be a good excuse to use one.)

## Raw Text
AKA: I spend 25% of my writeup discussing something that makes up 3% of a codebase
- Comments
- - These should be slightly faded such that they blend into the background. It's important that these are the least eye-catching out of anything in the color palette.
- - However, I'd ask that they don't fade into the background completely. Many colorschemes make the mistake of making them blend in *too* well, to the point where any long comment becomes unreadable without squinting. Having 5+ lines of comments is very plausible, and I dont want to strain my eyes reading them.
- Strings
- - Ideally, these are a fairly "basic" color. If a string stands out, it should only be by merit of being surrounded by variables. Tldr; distinct but not eye-catching.
- - Actually, it's very common for Strings to use a same or similar color to Constants (as defined in the first segment)!
- RegEx
- - A type of string with a unique format meant to concisely denote complex textual patterns - for example, "\(?\d{3}\)?-\d{3}-\d{4}" represents a well-formatted phone number.
- - These are pretty uncommon, so don't make this your favorite color (lol)
- - This should definitely contrast with the background. If the baseline string color already contrasts the background, it's fine to make this the same color.
- DocString (For my purposes, Comment docs as well)
- - A type of string in a unique, predetermined format, and meant to provide documentation on specific segments of code. It should be expected to see 5+ lines of nothing but DocStrings.
- - These are commonly a different color to standard strings, primarily because the color chosen for a standard string looks really really ugly when there's 5+ lines of nothing but it. If there's not a problem devoting a quarter of the screen to text of the base string color, then it's fine to leave this as the same color as a standard string too.
- Debug
- - A sequence of text relevant to debugging a program, rather than actually running it. Generally very concise, spanning ~30 characters and always existing on a line with nothing else on it.
- - Should be distinct from Declarations and any other form of Raw Text. Beyond that, a darker color is probably appropriate.
- Unique
- - A string used to represent data with a predetermined format (eg. the text "11/9/04" representing a date).
- - If RegEx is a different color from standard strings, you can re-use it here. If not, I'd prefer for the color to be unique. It still doesn't need to stand out among code, but it should be visibly different from a standard string.
- Path
- - A type of string representing a file on your computer.
- - Breaking the pattern established above, I *would* like this to be distinct from strings no matter what. If you're not using the color for Constants anywhere else, you could use it or a shade of it here.
- - These are also super sparse, so once again, don't reserve a great color just for this.
- - Optionally, I can include an underline underneath Path text. If you want to reuse a color completely, an underline can mean that it's still immediately distinct.
- Url
- - Text representing a url. These CAN show up in comments too - so I might ask for one shade for an in-string url, and one for an in-comment url.
- - Just like paths, I can underline these. You could totally just make this a very slightly lighter shade of string/comment, since the underline will ensure it's distinct.
- - No problem with reusing the color you used for paths on string-based Urls too.
- Escapes
- - A subsection of a string featuring unique or unconventional characters without standard keyboard representations - like `<BS>` to represent a backspace, or `\n` to represent a new line.
- - A lot of colorschemes make this an absurdly standout or jarring color, like a muted purple Escape inside a bright green String. I really don't understand why that is. I can't imagine why this wouldn't/couldn't/shouldn't just be a slightly lighter or off-color shade of whatever color the String is.
- - That said, it *is* important that escape characters should stand out no matter what type of string they're found in. (maybe that's why they're often really absurd colors..?)
- Character
- - A string composed of only one character. Kinda self explanatory.
- - Probably just a slightly off-color standard string. I dont think you need to overthink this one.

## Punctuation
AKA: The one exception to the rule of me not wanting to see the color white
- Bracket
- - Paired symbols like (), {}, [], etc.
- - These can show up anywhere, but are visually thin. You can get away with coloring these most anything as a highlight.
- - Extra credit if you can provide 2-3 colors for brackets, rather than just one. It helps to differentiate nested brackets (eg, having many (nested) parentheses in one another (like this (right here))) when there's more than one distinct color for them.
- - I've seen brackets with a gradient color scheme, where they get dimmer the more of them there are. You dont need to do that, but it's an option.
- Delimiter
- - You know what a delimiter is. These are tiny (,.;), so you once again get away with making them basically any (non-bracket, non-variable) color.
- Operator
- - Math symbols! +,-,=,!, and sometimes unique keywords like "and" or "not".
- - These can be pretty bright colors, or very dark colors. They'll always be nested between Variables and/or Literals, so pick a color that compliments both.
- - Most colorschemes go the easy way and pick an inoffensive color like a very very light blue. You probably could do this too
- - Since these (should) never be near any Declarations, you can totally reuse a color from there as well.
- Special
- - These are really annoying, because they need to be distinct from whatever context they're in - but the context they show up in varies by language. High contrast colors are preferred here.
- - Once again, you can totally reuse a color from the declarations segment here. 
- - It's very important that these are both visually distinct from, and pleasing to look at adjacent to whatever color you used for standard strings, as that is where Special punctuation most commonly shows up.
- - It's also very important that these look nothing alike to brackets' color(s), as characters like "{}" can be a Bracket or Special punctuation depending on the context. Thus, you should be able to tell its context from its color alone.

## General
- Underlines
- - ERROR (often neon red)
- - WARNING (often a very bright or very dark yellow)
- - INFO (often a blue... though I'd not want it to look similar to a Variable)
- Highlights
- - Generic cursor - my actual in-text cursor needs a shade too lol
- - text - i also need a color for the background of highlighted text
- - bracket matching - if my cursor is hovering over an open bracket, there should be a very striking highlight over the corresponding closed bracket.
- - - This can also come recolor the corresponding closed bracket itself, if you want.
- Diff
- - For use in comparing files to see what differences there are. Accomplished via automatically highlighting text.
- - PLUS - text present that didn't used to be. (Often some shade of green.)
- - MINUS - text that has been recently removed. (Often some shade of red.)
- - DELTA - text that was modified. (Nobody can agree what color this should be lmaoooooooooo)
