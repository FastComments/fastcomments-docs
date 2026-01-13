[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

בברירת המחדל, פונקציות העיצוב ב-FastComments מתבצעות על ידי הוספת תגי עוגן גלויים כמו `<b></b>` סביב הטקסט שלך. לחיצה על סרגל הכלים או שימוש בקיצורי מקשים עושה זאת עבורך. עם זאת, קהילות מסוימות עשויות לרצות לבחור להשתמש בעיצוב ללא תגי עוגן גלויים. זה נקרא הפעלת העורך WYSIWYG (מה שאתה רואה הוא מה שאתה מקבל). העורך הזה נראה בדיוק כמו העורך של ברירת המחדל, למעט שהוא טוען קצת קוד נוסף שמאפשר למשתמשים להדגיש, לקו תחתון וכו' את הטקסט שלהם ללא תגי עוגן גלויים.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

ניתן גם לעשות זאת ללא קוד. בדף ההתאמה האישית של הווידג'ט, ראה את האפשרות "הפעלת עיצוב מתקדם".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]