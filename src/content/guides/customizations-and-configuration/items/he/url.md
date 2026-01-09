[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

כששולחים הודעות דואר אלקטרוני על התראות, או מציגים תגובות בממשקי משתמש כמו דף המודרציה, זה מועיל להיות מסוגלים לקשר
מתוך התגובה אל הדף שבו היא נמצאת.

אם מזהה ה-URL אינו תמיד מזהה, אז עלינו לאחסן את ה-URL במקום אחר. לשם כך קיים המאפיין "url", המוגדר כדלקמן.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

מקרה שימוש נפוץ הוא קשירת שרשור התגובות למזהה, כמו מאמר, ולאחר מכן קישור חזרה לדף מסוים, לדוגמה:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

ה-URL אינו מנוקה מפרמטרים שיווקיים נפוצים. כברירת מחדל, ה-URL של הדף הנוכחי נשמר עם התגובה.

---