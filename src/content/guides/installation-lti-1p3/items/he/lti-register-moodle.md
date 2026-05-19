**משתמשים ב‑Moodle?** אנו מפרסמים גם תוסף Moodle ייעודי ל‑FastComments עם אינטגרציה צמודה יותר מ‑LTI 1.3 (hooks לסנכרון ציונים, דיווח פעילות מעמיק יותר, ממשק הגדרות מקומי ב‑Moodle). ראה את ה־<a href="/guide-installation-moodle.html" target="_blank">מדריך התקנת התוסף ל‑Moodle</a>. זרימת LTI 1.3 להלן היא הבחירה הנכונה אם ברצונך רישום יחיד המכסה גם LMSים אחרים, או אם מנהל ה‑Moodle שלך לא יתקין תוספי צד שלישי.

Moodle 4.0+ תומך ברישום דינמי של LTI 1.3 באמצעות התוסף External tool.

#### פתח את מסך ניהול הכלים

1. התחבר ל‑Moodle כמנהל האתר.
2. נווט אל **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### הדבק את ה-URL

תראה כרטיס שכותרתו **Tool URL**. הדבק את כתובת ההרשמה של FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">קבל אותה כאן</a>) לתוך שדה הטקסט ולחץ על **Add LTI Advantage**.

Moodle יפתח מסך רישום המציג את זהות הכלי ואת ההרשאות שהוא מבקש. בדוק ולחץ על **Activate** (או **Register**, בהתאם לגרסת Moodle).

חלון הקופץ ייסגר כאשר הרישום יסתיים; כלי FastComments החדש יופיע ברשימת **Tools** עם הסטטוס **Active**.

#### הפוך אותו לזמין

כברירת מחדל Moodle מוסיף כלים חדשים לרשימת "Course tools" אך אינו מציג אותם בבוחר הפעילויות. כדי לחשוף את FastComments בכל הקורס:

1. לחץ על סמל גלגל השיניים בכרטיס של FastComments.
2. מתחת ל‑**Tool configuration usage**, בחר **Show in activity chooser and as a preconfigured tool**.
3. שמור.

מרצים יכולים כעת להוסיף את FastComments לכל קורס דרך **Add an activity or resource** > **FastComments**.