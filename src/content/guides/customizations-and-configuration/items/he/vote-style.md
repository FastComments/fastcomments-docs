[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

בברירת מחדל, FastComments יציג אפשרויות הצבעה כחצים למעלה ולמטה, ומאפשר למשתמשים להצביע למעלה או למטה על תגובה.

עם זאת, ניתן לשנות את סגנון סרגל ההצבעה. האפשרויות הנוכחיות הן כפתורי העלייה/הירידה ברירת המחדל, או להשתמש במנגנון הצבעה בסגנון לב.

אנו משתמשים בדגל **voteStyle** כך:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'הפעלת כפתור לב'; code-example-end]

אנו ממליצים בחום לבצע זאת ללא קוד מכיוון שזה גם מאפשר אימותים בצד השרת. בעמוד התאמת הווידג'ט, ראה את סעיף "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='הגדרת סגנון הצבעה בעמוד התאמת הווידג\'ט, מציע חצים למעלה ולמטה או הצבעה בלב'; title='שינוי סגנון הצבעה' app-screenshot-end]

ניתן גם להשבית הצבעה, ראה `Disable Voting` מעל אפשרויות הסגנון.