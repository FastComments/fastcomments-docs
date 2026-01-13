בממשק ניהול ה-Webhooks יש כפתורי `Send Test Payload` עבור כל סוג אירוע (Create, Update, Delete). אירועי Create ו-Update שולחים אובייקט WebhookComment מדומה, בעוד שבבדיקת Delete ישלח גוף בקשה מדומה המכיל רק ID.

## אימות המטענים

בביצוע בדיקות של שילוב ה-webhook שלך, ודא שהבקשות הנכנסות כוללות את הכותרות הבאות:

1. **`token`** - סוד ה-API שלך
2. **`X-FastComments-Timestamp`** - חותמת זמן Unix (בשניות)
3. **`X-FastComments-Signature`** - חתימת HMAC-SHA256

השתמש באימות חתימת HMAC כדי לוודא שהמטענים אותנטיים.

## כלי בדיקה

ניתן להשתמש בכלים כמו [webhook.site](https://webhook.site) או [ngrok](https://ngrok.com) כדי לבדוק את מטעני ה-webhook הנכנסים במהלך הפיתוח.

## סוגי אירועים

- **Create Event**: מתרחש כאשר תגובה חדשה נוצרת. שיטת ברירת מחדל: PUT
- **Update Event**: מתרחש כאשר תגובה נערכת. שיטת ברירת מחדל: PUT
- **Delete Event**: מתרחש כאשר תגובה נמחקת. שיטת ברירת מחדל: DELETE

כל אירוע כולל את נתוני התגובה המלאים בגוף הבקשה (ראה [מבני נתונים](/guides/webhooks/webhooks-structures) עבור פורמט המטען).

---