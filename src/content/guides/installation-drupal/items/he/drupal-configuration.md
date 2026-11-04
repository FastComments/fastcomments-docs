כל ההגדרות נמצאות תחת `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## נדרש

- **מזהה דייר (Tenant ID)** - מזהה הדייר שלך ב-FastComments. ניתן למצוא אותו תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **סוד API (API Secret)** - נדרש עבור SSO מאובטח, אימות webhooks וסנכרון דפים. נמצא תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## אחסון סוד ה-API

המודול קורא את סוד ה-API מהמקור הראשון מבין אלה שיש לו ערך:

1. **מודול Key** - התקן את [Key](https://www.drupal.org/project/key), צור מפתח מסוג *Authentication*, ובחר אותו תחת **API Secret key** בטופס ההגדרות. המפתח יכול להתבסס על משתנה סביבה או קובץ מחוץ לשורש האתר.
2. **settings.php** - הוסף `$config['fastcomments.settings']['api_secret'] = 'your-secret';` ל-`settings.php` או ל-`settings.local.php`. הסוד לעולם לא נכנס לתצורה המיוצאת.
3. **טופס ההגדרות** - הזן אותו בשדה **API Secret**. פעולה זו שומרת את הערך ב-`fastcomments.settings`, ולכן הוא יופיע בייצואי התצורה. דוח המצב של Drupal מציג אזהרה כאשר זהו המקור הפעיל.

הטופס מציג איזה מקור פעיל. השארת השדה ריק שומרת על הסוד הקיים. סמן את **Remove the API Secret stored in configuration** כדי למחוק אותו לאחר העברת הסוד למודול Key או ל-`settings.php`.

## סגנון תגובות

בחר את הווידג'ט המתאים לאופן שבו אתה רוצה שאנשים ידברו באתר שלך.

- **Live Comments** - תגובות משורשרות בזמן אמת.
- **Streaming Chat** - ממשק צ'אט חי, מתאים לאירועים ולשידורים חיים.
- **Collab Chat** - הערות על בחירת טקסט באזור התוכן הראשי. מבקרים מסמנים טקסט ופותחים דיון בהקשר.
- **Collab Chat + Comments** - גם צ'אט שיתופי וגם תגובות רגילות באותו עמוד.

## מצב SSO

- **None** - ללא SSO. משתמשים מגיבים כאורחים או יוצרים חשבון FastComments.
- **Simple** - מעביר מידע על משתמש Drupal (שם, אימייל, אווטאר) ל-FastComments ללא אימות בצד השרת.
- **Secure** - משתמש ב-HMAC-SHA256 לאימות משתמשי Drupal מול FastComments. מומלץ כאשר מוגדר אצלך סוד API.

ראה את הסעיף `Single Sign-On (SSO)` לפרטים נוספים.

## הגדרות אחרות

- **CDN URL** - ברירת מחדל: `https://cdn.fastcomments.com`.
- **Site URL** - ברירת מחדל: `https://fastcomments.com`.
- **התראות אימייל** - שליחת אימייל למחבר התוכן כאשר מתפרסמת תגובה חדשה על התוכן שלו.

עבור אחסון נתונים ב-EU, ראה את הסעיף `EU Data Residency`.