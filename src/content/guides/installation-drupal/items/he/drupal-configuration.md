כל ההגדרות נמצאות תחת `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## נדרש

- **Tenant ID** - מזהה ה-Tenant של FastComments שלך. ניתן למצוא זאת תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - נדרש ל-Secure SSO, אימות webhook, וסינכרון דפים. נמצא תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## סגנון תגובות

בחר את הווידג'ט שמתאים לאופן שבו ברצונך שאנשים ידברו באתר שלך.

- **Live Comments** - תגובות מקוטלגות בחוטים בזמן אמת.
- **Streaming Chat** - ממשק צ'אט חי, מתאים לאירועים והזרמות ישירות.
- **Collab Chat** - הערות בהקשר באמצעות בחירת טקסט באזור התוכן הראשי. מבקרים מסמנים טקסט ומתחילים דיון בהקשר.
- **Collab Chat + Comments** - גם Collab Chat וגם תגובות סטנדרטיות באותו עמוד.

## מצב SSO

- **None** - אין SSO. משתמשים מגיבים כאורחים או יוצרים חשבון FastComments.
- **Simple** - מעביר מידע משתמש מחוץ ל-Drupal (שם, אימייל, אוואטר) ל-FastComments ללא אימות בצד השרת.
- **Secure** - משתמש ב-HMAC-SHA256 כדי לאמת משתמשי Drupal מול FastComments. מומלץ כאשר יש לך API Secret מוגדר.

ראה את הסעיף `Single Sign-On (SSO)` לפרטים.

## הגדרות נוספות

- **CDN URL** - ברירת מחדל היא `https://cdn.fastcomments.com`.
- **Site URL** - ברירת מחדל היא `https://fastcomments.com`.
- **Email notifications** - שלח אימייל למחבר התוכן כאשר תגובה חדשה מתפרסמת על התוכן שלו.

לנושא שהות נתונים באיחוד האירופי, ראה את הסעיף `EU Data Residency`.