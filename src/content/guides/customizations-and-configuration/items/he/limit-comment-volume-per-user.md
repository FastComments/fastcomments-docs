בברירת מחדל, כל משתמש יכול לשלוח עד `5 comments` באותה דקה.

המעקב נעשה באמצעות user id, anon user id וip address (hashed).

ניתן להתאים זאת ללא קוד, בדף התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

שים לב שאם אתה משתמש ב- comment creation API, ייתכן שתרצה להעביר את כתובת ה- `ip` המקורית של המשתמש בבקשה אל ה-backend שלנו כדי שמגבלת התדירות תחול
לפי משתמש ולא באופן גלובלי על חשבונך.