[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

בברירת מחדל, FastComments יציג את אפשרויות ההצבעה כחיצים למעלה ולמטה, מה שמאפשר למשתמשים להצביע בעד או נגד תגובה.

עם זאת, ניתן לשנות את סגנון סרגל ההצבעה. האפשרויות הנוכחיות הן כפתורי ברירת המחדל למעלה/למטה, או שימוש במנגנון הצבעה בסגנון לב.

אנו משתמשים בדגל **voteStyle** כך:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

אנו ממליצים בחום לבצע זאת ללא קוד, שכן זה גם מאפשר אימותים בצד השרת. בעמוד התאמת הווידג'ט, ראה את הסעיף "סגנון ההצבעה".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

ניתן גם להשבית את ההצבעה — ראה את `Disable Voting` מעל אפשרויות הסגנון.

---