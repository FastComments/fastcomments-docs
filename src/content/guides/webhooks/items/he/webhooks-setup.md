עקוב אחר אותם השלבים עבור `localhost` כפי שתעשה עבור production. וודא שיש לך production domains ו-API Secrets מוגדרים.

ראשית, נווט אל [מנהל Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). ניתן לגשת אל דף זה דרך Manage Data -> Webhooks.

דף התצורה נראה כך:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='תצורת Webhooks' app-screenshot-end]

בעמוד זה ניתן לציין endpoints עבור כל סוג של אירוע תגובה.

עבור כל סוג של אירוע, הקפד ללחוץ על Send Test Payload כדי לוודא שהאינטגרציה הוגדרה כהלכה. ראה את הסעיף הבא, "בדיקה", לפרטים.

---