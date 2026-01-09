[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

ברירת המחדל של FastComments היא למיין את התגובות לפי כיוון המיון "הכי רלוונטי".

מיון "הכי רלוונטי" לוקח בחשבון את זמן השארת התגובה ואת מספר ההצבעות בעת המיון.

המשתמש יכול לשנות את כיוון המיון ל־'הישן ראשון' או 'החדש ראשון' בממשק הווידג'ט של התגובות.

עם זאת, ניתן לשנות את ברירת המחדל לאחת משלוש האפשרויות. לדוגמה, אם תרצו להציג את התגובות הוותיקות ביותר תחילה:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

קבענו את הערך של **defaultSortDirection** כ־"OF" כדי להגדיר את כיוון המיון כ־"OF".

עבור כיוון המיון של 'החדש ראשון' נעשה את הפעולה הבאה:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

הערכים התקפים עבור **defaultSortDirection** הם:

- MR: "העדכני ביותר"
- NF: "החדש ראשון"
- OF: "הישן ראשון"

ניתן גם לבצע זאת ללא קוד. בדף התאמת הווידג'ט, ראו את הסעיף "כיוון מיון ברירת מחדל".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

שימו לב שהתגובות בכל דף עבור כל כיוון מיון מחושבות מראש, ולכן לכל כיווני המיון ביצועים זהים.