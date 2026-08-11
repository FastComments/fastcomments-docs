[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

לאימות, FastComments תלויה בעוגיות צד שלישי המופעלות בדפדפן שלך. ללא עוגיות אלו, המשתמשים תמיד יצטרכו להשאיר את האימייל שלהם כדי להגיב (אלא אם שדה הקלט של האימייל מוסתר), והתגובות שלהם תמיד יופיעו כלא מאומתות (בברירת מחדל).

כדי לעקוף זאת, ניתן להפעיל את עקיפת קוביית צד שלישי. 

כאשר הגדרה זו מופעלת, היא תגרום לתצוגת פופ‑אפ קטן שמציג הודעה שהמשתמש מתבצע כניסה. פופ‑אפ זה מופיע בכל פעם שהמשתמש מקיים אינטראקציה עם וידג'ט ההערות; לדוגמה, אם הוא משאיר תגובה.

ניתן לבצע זאת בקוד על‑ידי הגדרת הדגל **enableThirdPartyCookieBypass** ל‑true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'הפעלת עקיפת קוביית צד שלישי'; code-example-end]

ניתן גם להגדיר זאת דרך ממשק התאמת הווידג'ט, תחת `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='דף התאמת הווידג\'ט עם תיבת הסימון של הפעלת חלון קוביית צד שלישי מסומנת'; title='הפעלת עקיפת קוביית צד שלישי' app-screenshot-end]