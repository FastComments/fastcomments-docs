D2L Brightspace חושפת רישום דינמי דרך ממשק הניהול של LTI Advantage. תזדקק לגישת מנהל.

#### פתח את מסך הרישום

1. היכנס/י למופע Brightspace שלך כמנהל.
2. נווט/י אל **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. לחץ/י על **Register Tool**. (The direct URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### הדבק את ה־URL

תראה טופס רישום. השדה המרכזי הוא **Tool initiation registration endpoint** (בגרסאות מסוימות של Brightspace הוא מתויג כ-"Tool Initiation Registration URL").

הדבק את כתובת ה‑URL של רישום FastComments בשדה הזה. השאר את שאר השדות ריקים — FastComments ימלא אותם אוטומטית במהלך החלפת המפתחות של הרישום.

לחץ/י על **Register**.

#### אשר את הכלי

Brightspace פותחת חלון קופץ שמתקשר עם FastComments, מחליף מפתחות ומציג מסך אישור. חלון הקופץ נסגר אוטומטית עם השלמת הרישום.

הכלי החדש מופיע ברשימת כלי LTI Advantage שלך. כברירת מחדל Brightspace מסמנת כלים חדשים כ**disabled** — החלף/י את המתג ל-**enabled** כדי שהקורסים שלך יוכלו להשתמש בו.

#### הוספת פריסה

ב‑Brightspace, לכלי LTI צריך להיות **deployment** לפני שניתן להשתמש בהם בקורסים:

1. פתח/י את כלי FastComments שנרשם זה הרגע.
2. לחץ/י על **View Deployments** > **New Deployment**.
3. תן/י שם לפריסה (לדוגמה "FastComments - All Courses"), בחר/י את יחידות הארגון שבהן היא אמורה להיות זמינה, ושמור/י.

לאחר ההשקה הראשונה דרך פריסה זו, FastComments מצמיד את ה־`deployment_id` לרשומת התצורה שלה — השקות מאוחרות יותר מפריסה שונה תחת אותו לקוח יידחו אלא אם תירשם/י מחדש.