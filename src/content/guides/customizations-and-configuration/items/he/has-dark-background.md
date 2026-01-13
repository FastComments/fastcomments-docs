---
[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

כברירת מחדל, וידג'ט ההערות של FastComments יזהה אוטומטית מצב כהה ברוב האתרים.

כאשר מזוהה מצב כהה, FastComments יהפוך טקסט שחור על רקעים לבנים לטקסט לבן על רקע שחור. גם תמונות ישתנו.

בעת טעינת הדף, הווידג'ט ינסו לקבוע עד כמה רקע הדף מאחורי הווידג'ט כהה. משמעות הדבר היא ש
הדף יכול להיות עם רקע לבן, אך אם תשים את הווידג'ט של ההערות בתוך מכל (container) עם רקע שחור, מצב כהה אמור
עדיין להיות מופעל אוטומטית כדי להקל על קריאת ההערות.

עם זאת, מנגנון הגילוי, שתלוי בקביעת "בהירות", עשוי שלא להפעיל את מצב-הכהה כאשר תרצה בכך. כדי לאלץ הפעלה שלו, הגדר את
*hasDarkBackground* flag to true as follows:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---