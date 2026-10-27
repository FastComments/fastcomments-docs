All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## נדרש

- **Tenant ID** - מזהה השוכר שלך ב‑FastComments. מצא זאת תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - נדרש עבור Secure SSO, אימות webhook, וסנכרון דף. נמצא תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## שמירת סוד ה‑API

The module reads the API Secret from the first of these sources that has a value:

1. **Key module** - התקן את [Key](https://www.drupal.org/project/key), צור מפתח *Authentication*, ובחר אותו תחת **API Secret key** בטופס ההגדרות. המפתח יכול להיות מגובה במשתנה סביבתי או בקובץ מחוץ לשורש האינטרנט.
2. **settings.php** - הוסף `$config['fastcomments.settings']['api_secret'] = 'your-secret';` ל‑`settings.php` או `settings.local.php`. הסוד לעולם לא נכנס לתצורת הייצוא.
3. **The settings form** - הזן אותו בשדה **API Secret**. זה מאחסן את הערך ב‑`fastcomments.settings`, ולכן הוא מופיע בייצואי התצורה. דוח המצב של Drupal מזהיר כאשר זהו המקור הפעיל.

The form shows which source is active. Leaving the field blank keeps the stored secret. Tick **Remove the API Secret stored in configuration** to delete it after moving the secret to the Key module or `settings.php`.

## סגנון תגובות

Pick the widget that matches how you want people to talk on your site.

- **Live Comments** - תגובות בזמן אמת עם שרשור.
- **Streaming Chat** - ממשק צ'אט חי, טוב לאירועים ושידורים חיים.
- **Collab Chat** - אנוטציה של בחירת טקסט באזור התוכן הראשי. מבקרים מדגישים טקסט ומתחילים דיון בהקשר.
- **Collab Chat + Comments** - גם צ'אט שיתופי וגם תגובות סטנדרטיות באותה העמוד.

## מצב SSO

- **None** - ללא SSO. משתמשים מגיבים כאורחים או יוצרים חשבון FastComments.
- **Simple** - מעביר מידע משתמש של Drupal (שם, אימייל, תמונת פרופיל) ל‑FastComments ללא אימות בצד השרת.
- **Secure** - משתמש ב‑HMAC‑SHA256 כדי לאמת משתמשי Drupal עם FastComments. מומלץ כאשר סוד ה‑API מוגדר.

See the `Single Sign-On (SSO)` section for details.

## הגדרות נוספות

- **CDN URL** - ברירת מחדל היא `https://cdn.fastcomments.com`.
- **Site URL** - ברירת מחדל היא `https://fastcomments.com`.
- **Email notifications** - שלח אימייל למחבר התוכן כאשר תגובה חדשה מתפרסמת על התוכן שלו.

For EU data residency, see the `EU Data Residency` section.