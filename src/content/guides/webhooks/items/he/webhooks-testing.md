בממשק הניהול של Webhooks יש כפתורי `Send Test Payload` עבור כל סוג אירוע (Create, Update, Delete). אירועי Create ו-Update שולחים אובייקט WebhookComment מדומה, בעוד שבדיקת Delete תשלח גוף בקשה מדומה עם מזהה בלבד.

## אימות המטענים

בעת בדיקת אינטגרציית ה-webhook, וודא שהבקשות הנכנסות כוללות את הכותרות הבאות:

1. **`token`** - סוד ה-API שלך
2. **`X-FastComments-Timestamp`** - חותמת זמן Unix (בשניות)
3. **`X-FastComments-Signature`** - חתימת HMAC-SHA256

יש להשתמש באימות חתימת HMAC כדי לוודא שהמטענים אותנטיים.

## כלי בדיקה

ניתן להשתמש בכלים כמו [webhook.site](https://webhook.site) או [ngrok](https://ngrok.com) כדי לבדוק את מטעני ה-webhook הנכנסים בזמן פיתוח.

## סוגי אירועים

- **Create Event**: מופעל כאשר נוצרת תגובה חדשה. שיטת ברירת המחדל: PUT
- **Update Event**: מופעל כאשר תגובה נערכת. שיטת ברירת המחדל: PUT
- **Delete Event**: מופעל כאשר תגובה נמחקת. שיטת ברירת המחדל: DELETE

כל אירוע כולל את כל נתוני התגובה בגוף הבקשה (ראה [מבני נתונים](/guide-webhooks.html#webhooks-structures) עבור פורמט המטען).