Blackboard Learn SaaS ו-Ultra תומכים ברישום דינמי של LTI 1.3.

#### פתח את מסך ספק הכלי

1. היכנס ל-Blackboard כמנהל מערכת.
2. נווט אל **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. לחץ על **Register LTI 1.3 / LTI Advantage Tool**.

אם אתה רואה רק "Register LTI 1.1 Provider", גרסת Blackboard שלך עדיין לא תומכת ב-LTI 1.3 — שדרג או פנה לתמיכת Blackboard.

#### הדבק את ה-URL

הדבק את כתובת ההרשמה של FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">השג אותה כאן</a>) אל שדה **Client ID** / **Registration URL** (תוויות ב-Blackboard משתנות בין גרסאות). שלח.

Blackboard מבצע את תהליך ה-handshake של ההרשמה עם FastComments ומציג מסך אישור.

#### אשר והפעל

Blackboard מסמן כלים שנרשמו זה עתה כברירת מחדל כ-**Approved but excluded**:

1. מצא את הרשומה של FastComments ברשימת ספקי הכלים.
2. פתח את התפריט ובחר **Edit**.
3. הגדר את **Tool Status** ל-**Approved**.
4. תחת **Institution Policies**, עיין אילו נתוני משתמש נשלחים (שם, דוא"ל, תפקיד). שמור.

הכלי כעת זמין למרצים כאשר הם מוסיפים תוכן לקורסים.