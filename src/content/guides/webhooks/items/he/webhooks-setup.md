---
עקוב אחר אותם שלבים עבור `localhost` כפי שתעשה ב-production. ודא שיש לך production domains ו-API Secrets מוגדרים.

ראשית, נווט אל ה-[Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks). ניתן לגשת לכך דרך Manage Data -> Webhooks.

דף התצורה נראה כך:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration'; cacheBuster = 'v3' app-screenshot-end]

בעמוד זה ניתן לציין endpoints עבור כל סוג של אירוע תגובה.

עבור כל סוג אירוע, הקפד ללחוץ על Send Test Payload כדי לוודא שהאינטגרציה הוגדרה כראוי. ראה את הסעיף הבא, "Testing", לפרטים.
---