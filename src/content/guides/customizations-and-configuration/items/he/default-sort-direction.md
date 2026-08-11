[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

בברירת מחדל, FastComments ימיין תגובות לפי כיוון המיון "הכי רלוונטי".

מיון הכי רלוונטי מתחשב בזמן שהתגובה נכתבה ובמספר הקולות לצורך המיון.

המשתמש יכול לאחר מכן לשנות את כיוון המיון ל"ישן ראשון" או "חדש ראשון" בממשק הווידג'ט של התגובות.

עם זאת, ניתן לשנות את ברירת המחדל לכל אחד משלושת האפשרויות. לדוגמה, אם ברצונך להציג את התגובות הישנות ביותר ראשונות:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'שינוי המיון ברירת המחדל לישן ראשון'; code-example-end]

אנו מגדירים את ערך **defaultSortDirection** ל-"OF" כדי לקבוע את הכיוון ל-"OF".

לכיוון המיון "חדש ראשון", נבצע את הפעולה הבאה:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'שינוי המיון ברירת המחדל לחדש ראשון'; code-example-end]

הערכים החוקיים עבור **defaultSortDirection** הם:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

ניתן לבצע זאת גם ללא קוד. בעמוד התאמת הווידג'ט, ראה את הסעיף "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='בוחר כיוון המיון ברירת המחדל המציע רלוונטיות מרבית, חדש ראשון, וישן ראשון'; title='שינוי כיוון המיון ברירת המחדל' app-screenshot-end]

שימו לב, שהתגובות בכל דף עבור כל כיוון מיון מחושבות מראש, ולכן לכל כיווני המיון יש את אותה ביצועים.