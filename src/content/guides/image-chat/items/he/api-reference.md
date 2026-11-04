### סקירת ה-API

Image Chat מספק שלושה נקודות קצה (endpoints) של REST API לניהול שיחות תמונה באופן תוכנתי. נקודות קצה אלו מאפשרות לך לאחזר, ליצור ולמחוק שיחות מבלי להשתמש בווידג'ט הדפדפן.

כל נקודות הקצה של ה-API דורשות אימות ופועלות לפי התבניות הסטנדרטיות של FastComments API. אלו נקודות קצה ציבוריות שמאומתות באמצעות עוגיות דפדפן, ולא באמצעות מפתחות API.

### כתובת בסיס (Base URL)

כל נקודות הקצה של Image Chat API נמצאות תחת:

```
https://fastcomments.com/comment-image-chats
```

### אימות

נקודות קצה אלו מאמתות משתמשים באמצעות עוגיות דפדפן. הן אינן משתמשות במפתחות API. על המשתמשים להיות מחוברים ל-FastComments בדפדפן שלהם כדי לגשת לנקודות קצה אלו.

### קבלת כל השיחות

אחזר את כל שיחות התמונה עבור תמונה ספציפית.

#### נקודת קצה

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

`urlId` (פרמטר שאילתה, נדרש) הוא מזהה התמונה שעבורה ברצונך לאחזר שיחות.

#### תגובה

התגובה כוללת את סטטוס ה-API, מידע על סשן המשתמש הנוכחי אם האימות בוצע, מערך של שיחות עם המזהים, ה-URLs וקואורדינטות ה-X/Y שלהן, מזהה URL מנוקה, דגל המציין אם המשתמש הנוכחי הוא מנהל אתר או מנחה, ופרטי חיבור WebSocket לסנכרון חי כולל `tenantIdWS`, `urlIdWS` ו-`userIdWS`.

#### דוגמת בקשה

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### דוגמת תגובה

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### יצירת שיחה

צור שיחת תמונה חדשה עבור מיקום ספציפי בתמונה.

#### נקודת קצה

```
POST /comment-image-chats/:tenantId
```

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

גוף הבקשה חייב להיות JSON ולכלול את השדות הנדרשים הבאים.

`urlId` (מחרוזת, נדרש) הוא מזהה העמוד הבסיסי.

`windowUrlId` (מחרוזת, נדרש) הוא ה-URL המשולב עם מקור התמונה והקואורדינטות, לדוגמה `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (מחרוזת, נדרש) הוא כותרת העמוד.

`src` (מחרוזת, נדרש) הוא כתובת ה-URL של מקור התמונה.

`x` (מספר, נדרש) הוא קואורדינטת ה-X כאחוז (0-100).

`y` (מספר, נדרש) הוא קואורדינטת ה-Y כאחוז (0-100).

`createdFromCommentId` (מחרוזת, נדרש) הוא המזהה של התגובה שיזמה את השיחה הזו.

`broadcastId` (מחרוזת, נדרש) הוא UUID לסנכרון חי למניעת אפקטי הדהוד.

#### תגובה

התגובה כוללת את סטטוס ה-API ואת המזהה של השיחה החדשה שנוצרה.

#### דוגמת בקשה

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### דוגמת תגובה

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### מחיקת שיחה

מחק שיחת תמונה קיימת. נקודת קצה זו דורשת הרשאות מנהל או מנחה, או שהשיחה חייבת להיות נוצרה על ידי המשתמש המאומת.

#### נקודת קצה

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

`chatId` (פרמטר נתיב, נדרש) הוא המזהה של השיחה שיש למחוק.

`broadcastId` (גוף הבקשה, נדרש) הוא UUID לסנכרון חי.

#### דוגמת בקשה

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### דוגמת תגובה

```json
{
  "status": "success"
}
```

### מערכת הקואורדינטות

קואורדינטות ה-X וה-Y נשמרות כאחוזים מממדי התמונה. X מייצג את המיקום האופקי מהקצה השמאלי (0 = הקצה השמאלי, 100 = הקצה הימני). Y מייצג את המיקום האנכי מהקצה העליון (0 = הקצה העליון, 100 = הקצה התחתון).

ערכי האחוזים יכולים לכלול מספרים עשרוניים לדיוק. לדוגמה, x: 25.5 פירושו 25.5% מהקצה השמאלי של התמונה.

### הגבלת קצב (Rate Limiting)

נקודות קצה אלו כפופות להגבלת הקצב הסטנדרטית של FastComments API. תוכנת הביניים (middleware) להגבלת קצב חלה פר-Tenant כדי למנוע שימוש לרעה תוך מתן אפשרות לדפוסי שימוש רגילים.

### תגובות שגיאה

כל נקודות הקצה מחזירות קודי סטטוס HTTP סטנדרטיים. תגובה 400 מציינת פרמטרי בקשה לא תקינים. תגובה 401 פירושה שהאימות נכשל. תגובה 403 מציינת הרשאות לא מספיקות. תגובה 404 פירושה שהשיחה לא נמצאה. תגובה 429 מציינת שמגבלת הקצב חורגה.

תגובות שגיאה כוללות גוף JSON עם פרטים:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### מעקב שימוש

יצירת שיחות מגדילה את מדד השימוש `conversationCreateCount` שלך. כל פעילות סנכרון WebSocket מגדילה את `pubSubMessageCount` ו-`pubSubBandwidth`. באפשרותך לעקוב אחר מדדים אלו בלוח הבקרה של FastComments שלך תחת ניתוחי שימוש.