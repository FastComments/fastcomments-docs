הדפים החדשים ועמודי עריכת ה-webhook כוללים כפתור `Send Test Payload` ששולח בקשה לכתובת ה-URL הנוכחית בטופס, בין אם נשמרה ובין אם לא. אירועי ה-Create וה-Update שולחים אובייקט WebhookComment דמה, בעוד שבדיקת Delete תשלח גוף בקשה דמה הכולל רק מזהה (ID).

## אימות Payloads

בעת בדיקת אינטגרציית ה-webhook שלכם, ודאו שהבקשות הנכנסות כוללות את הכותרות הבאות:

1. **`X-FastComments-Timestamp`** - חותמת זמן Unix (שניות)
2. **`X-FastComments-Signature`** - חתימת HMAC-SHA256

Webhooks שנוצרו לפני הצגת סכמת החתימה מקבלים גם כותרת **`token`** המכילה את ה-API Secret שלכם. Webhooks חדשים לא.

השתמשו באימות חתימת ה-HMAC כדי לוודא שה-payloads אמיתיים.

## כלי בדיקה

ניתן להשתמש בכלים כמו [webhook.site](https://webhook.site) או [ngrok](https://ngrok.com) לבחינת payloads נכנסים של webhook במהלך הפיתוח.

## סוגי אירועים

- **אירוע Create**: מופעל כאשר נוצרת תגובה חדשה.
- **אירוע Update**: מופעל כאשר תגובה נערכת.
- **אירוע Delete**: מופעל כאשר תגובה נמחקת.

כל webhook קשור לאירוע אחד ולשיטת HTTP אחת (POST, PUT או DELETE). כל אירוע כולל את נתוני התגובה המלאים בגוף הבקשה (ראו [מבני נתונים](/guide-webhooks.html#webhooks-structures) לפורמט ה-payload).