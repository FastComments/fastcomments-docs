---
[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

לצורך אימות, FastComments מסתמכת על קוקיות של צד שלישי שמופעלות בדפדפן שלך. בלעדיהן, משתמשים תמיד יצטרכו להשאיר את האימייל שלהם כדי להוסיף תגובה (אלא אם שדה הקלט של האימייל מוסתר), והתגובות שלהם תמיד יוצגו כלא מאומתות (כברירת מחדל).

כדי לעקוף זאת, ניתן להפעיל את האפשרות לעקיפת קוקיות צד שלישי.

כאשר הגדרה זו מופעלת, תופיע חלונית קופצת קטנה שמציגה הודעה המודיעה שהמשתמש נכנס למערכת. חלונית זו תופיע בכל פעם שהמשתמש יפעיל את ווידג'ט התגובות; למשל, אם הוא ישאיר תגובה.

ניתן לעשות זאת בקוד על ידי הגדרת הדגל **enableThirdPartyCookieBypass** ל-true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

ניתן גם להגדיר זאת דרך ממשק התאמת הווידג'ט, תחת `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---