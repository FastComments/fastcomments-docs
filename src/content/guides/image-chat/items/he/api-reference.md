### API Overview

Image Chat מספק שלוש נקודות קצה של REST API לניהול שיחות תמונה באופן תכנותי. נקודות קצה אלו מאפשרות לך לשלוף, ליצור ולמחוק סימונים ללא שימוש בווידג'ט בדפדפן.

כל נקודות הקצה של ה-API דורשות אימות ועוקבות אחר דפוסי ה-API הסטנדרטיים של FastComments. אלה נקודות קצה ציבוריות שמאמתות באמצעות קוקיות בדפדפן, ולא באמצעות מפתחות API.

### Base URL

כל נקודות הקצה של Image Chat נמצאות תחת:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

נקודות קצה אלה מאמתות משתמשים באמצעות קוקיות בדפדפן. אינן משתמשות במפתחות API. על המשתמשים להיות מחוברים ל-FastComments בדפדפן שלהם כדי לגשת לנקודות קצה אלו.

### Get All Conversations

שלוף את כל שיחות התמונה עבור תמונה ספציפית.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

`urlId` (query parameter, required) הוא מזהה התמונה עבורה ברצונך לשלוף שיחות.

#### Response

התשובה כוללת את סטטוס ה-API, מידע על סשן המשתמש הנוכחי אם מאומת, מערך שיחות עם מזהים, כתובות URL וקואורדינטות X/Y, מזהה URL מנוקה, דגל המציין אם המשתמש הנוכחי הוא מנהל אתר או מודרטור, ופירוט חיבור WebSocket לסינכרון חי כולל `tenantIdWS`, `urlIdWS` ו-`userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

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

### Create Conversation

צור שיחת תמונה חדשה לנקודה מסוימת בתמונה.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

גוף הבקשה חייב להיות JSON ולכלול את השדות הנדרשים הבאים.

`urlId` (string, required) הוא מזהה הדף הבסיסי.

`windowUrlId` (string, required) הוא ה-URL המשולב עם מקור התמונה והקואורדינטות, לדוגמה `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) הוא הכותרת של הדף.

`src` (string, required) הוא כתובת המקור של התמונה.

`x` (number, required) הוא קואורדינטת X כאחוז (0-100).

`y` (number, required) הוא קואורדינטת Y כאחוז (0-100).

`createdFromCommentId` (string, required) הוא המזהה של ההערה שהחלה את השיחה הזו.

`broadcastId` (string, required) הוא UUID לשם סינכרון חי למניעת אפקטי הדהוד.

#### Response

התשובה כוללת את סטטוס ה-API ואת מזהה השיחה שנוצרה.

#### Example Request

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

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

מחק שיחת תמונה קיימת. נקודה זו דורשת הרשאות מנהל או מודרטור, או שהשיחה חייבת להיות נוצרה על ידי המשתמש המחובר.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

`chatId` (path parameter, required) הוא המזהה של השיחה שברצונך למחוק.

`broadcastId` (request body, required) הוא UUID לשם סינכרון חי.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

קואורדינטות X ו-Y מאוחסנות כאחוזים מממדי התמונה. X מייצג את המיקום האופקי מהקצה השמאלי (0 = קצה שמאלי, 100 = קצה ימני). Y מייצג את המיקום האנכי מהקצה העליון (0 = קצה עליון, 100 = קצה תחתון).

ערכי האחוזים הללו יכולים לכלול נקודות עשרוניות עבור דיוק. לדוגמה, x: 25.5 פירושו 25.5% מהקצה השמאלי של התמונה.

### Rate Limiting

נקודות קצה אלו כפופות להגבלת תדרים (rate limiting) הסטנדרטית של FastComments. ה-middleware של הגבלת התדרים פועל per-tenant כדי למנוע שימוש לרעה תוך כדי מתן אפשרות לתבניות שימוש רגילות.

### Error Responses

כל נקודות הקצה מחזירות קודי סטטוס HTTP סטנדרטיים. תגובת 400 מציינת פרמטרי בקשה לא תקינים. תגובת 401 משמעותה שהאימות נכשל. תגובת 403 מציינת הרשאות לא מספקות. תגובת 404 משמעותה שהשיחה לא נמצאה. תגובת 429 מציינת חריגה ממגבלת התדר.

תגובות שגיאה כוללות גוף JSON עם פרטים:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

יצירת שיחות מגדילה את מדד השימוש שלך `conversationCreateCount`. כל פעילות סינכרון WebSocket מגדילה את `pubSubMessageCount` ו-`pubSubBandwidth`. תוכל לנטר מדדים אלו בלוח המחוונים של FastComments תחת ניתוחי שימוש.