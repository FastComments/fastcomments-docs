[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

בברירת מחדל, הפונקציות של עיצוב ב‑FastComments מתבצעות על‑ידי הוספת תגי עוגן גלויים כמו `<b></b>` סביב הטקסט שלך. לחיצה על סרגל הכלים
או שימוש בקיצורים עושה זאת עבורך. עם זאת, קהילות מסוימות עשויות לרצות להשתמש בעיצוב ללא תגי עוגן. זה נקרא הפעלת
WYSIWYG (what you see is what you get) editor. עורך זה נראה בדיוק כמו העורך ברירת המחדל, אלא שהוא טוען קוד נוסף
המאפשר למשתמשים להדגיש, לקו תחתי, וכו' את הטקסט שלהם ללא תגי עוגן גלויים.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'הפעלת עריכת WYSIWYG'; code-example-end]

זה גם ניתן לבצע ללא קוד. בעמוד התאמה אישית של הווידג'ט, ראה את האפשרות "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='דף התאמה אישית של הווידג\'ט עם תיבת הסימון Enable Advanced Formatting מסומנת כדי להפעיל את עורך ה‑WYSIWYG'; title='הפעלת WYSIWYG' app-screenshot-end]