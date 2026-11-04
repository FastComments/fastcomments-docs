D2L Brightspace חושפת רישום דינמי (Dynamic Registration) דרך ממשק הניהול של LTI Advantage. תזדקקו לגישת מנהל מערכת.

#### פתיחת מסך הרישום

1. התחברו למופע ה-Brightspace שלכם כמנהל מערכת.
2. נווטו אל **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. לחצו על **Register Tool**. (כתובת ה-URL הישירה היא `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### הדבקת ה-URL

תראו טופס רישום. השדה המרכזי הוא **Tool initiation registration endpoint** (בגרסאות מסוימות של Brightspace הוא מסומן כ-"Tool Initiation Registration URL").

הדביקו את כתובת ה-URL לרישום של FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ניתן להשיג אותה כאן</a>) בשדה זה. השאירו את שאר השדות ריקים - הם מאוכלסים אוטומטית על ידי FastComments במהלך תהליך הרישום.

לחצו על **Register**.

#### אישור הכלי

Brightspace פותח חלון קופץ שמתקשר עם FastComments, מחליף מפתחות ומציג מסך אישור. החלון הקופץ נסגר מעצמו כאשר הרישום מושלם.

הכלי החדש יופיע ברשימת הכלים של LTI Advantage. כברירת מחדל, Brightspace מסמן כלים חדשים כ**מושבתים** - הפעילו את המתג ל**מופעל** כדי שהקורסים שלכם יוכלו להשתמש בו.

#### הוספת פריסה (Deployment)

ב-Brightspace, כלי LTI זקוקים ל**פריסה** (deployment) לפני שניתן יהיה להשתמש בהם בקורסים:

1. פתחו את הכלי החדש של FastComments שנרשם.
2. לחצו על **View Deployments** > **New Deployment**.
3. תנו לפריסה שם (למשל "FastComments - All Courses"), בחרו את יחידות הארגון שבהן היא תהיה זמינה, ושמרו.

לאחר ההפעלה הראשונה דרך פריסה זו, FastComments מקבע את ה-`deployment_id` ברשומת התצורה שלו - הפעלות נוספות מפריסה אחרת תחת אותו לקוח יידחו אלא אם תבצעו רישום מחדש.