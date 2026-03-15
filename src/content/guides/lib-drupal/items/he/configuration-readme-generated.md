נווט אל **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### הגדרות

- **Tenant ID** (required) - מזהה ה‑Tenant של FastComments שלך. ניתן למצוא אותו תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - נדרש עבור Secure SSO, אימות webhook ושימור/סנכרון דפים. נמצא תחת [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **SSO Mode** - אינטגרציית Single Sign-On:
  - **None** - ללא SSO, משתמשים מגיבים כאורחים או יוצרים חשבונות FastComments.
  - **Simple** - מעביר מידע משתמש מדראופל (שם, אימייל, אווטאר) אל FastComments ללא אימות בצד השרת.
  - **Secure** - משתמש באימות HMAC-SHA256 לאימות מאובטח של משתמשי Drupal מול FastComments (מומלץ).
- **Commenting Style** - סוג הווידג'ט להצגה:
  - **Live Comments** - תגובות מקוננות בזמן אמת.
  - **Streaming Chat** - ממשק צ'אט בזמן אמת.
  - **Collab Chat** - הערות שיתופיות באמצעות בחירת טקסט על אזור התוכן הראשי.
  - **Collab Chat + Comments** - גם Collab Chat וגם תגובות סטנדרטיות.
- **CDN URL** - כתובת CDN של FastComments (ברירת מחדל: `https://cdn.fastcomments.com`).
- **Site URL** - כתובת האתר של FastComments (ברירת מחדל: `https://fastcomments.com`).
- **Email notifications** - שלח דוא"ל למחברי התוכן כאשר תגובה חדשה מתפרסמת בתוכן שלהם.

### הוספת תגובות לסוגי תוכן

הוסף את השדה **FastComments** לסוגי התוכן שלך דרך **Structure > Content types > [type] > Manage fields**. לשדה יש מתג מצב ומזהה מותאם אישית אופציונלי לכל ישות.

### שמירת נתונים באיחוד האירופי

עבור שמירת נתונים באיחוד האירופי, עדכן:
- **CDN URL** ל- `https://cdn-eu.fastcomments.com`
- **Site URL** ל- `https://eu.fastcomments.com`