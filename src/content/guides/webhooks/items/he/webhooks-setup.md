עקבו אחרי אותם הצעדים עבור `localhost` כפי שהייתם עושים בייצור. ודאו שיש לכם תחומי ייצור והגדרות סודות API.

ראשית, נווטו אל [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). ניתן לגשת לכך דרך Manage Data -> Webhooks.

דף התצורה מופיע כך:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='דף ניהול Webhooks עם בורר תחום ושדה כתובת URL של נקודת קצה לכל אירוע תגובה, בנוסף כפתור שליחת מטען בדיקה'; title='תצורת Webhooks'; cacheBuster = 'v3' app-screenshot-end]

בדף זה ניתן לציין נקודות קצה לכל סוג של אירוע תגובה.

עבור כל סוג של אירוע, הקפידו ללחוץ על Send Test Payload כדי לוודא שהגדרתם את האינטגרציה כראוי. ראו את הסעיף הבא, "Testing", לפרטים.