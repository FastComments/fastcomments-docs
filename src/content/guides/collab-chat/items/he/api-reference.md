### סקירת API

Collab Chat מספקת שלוש נקודות קצה REST API לניהול שיחות טקסט באופן תכנותי. נקודות קצה אלה מאפשרות לך לאחזר, ליצור ולמחוק הערות מבלי להשתמש בווידג'ט בדפדפן.

אלו נקודות קצה ציבוריות שמאמתות משתמשים באמצעות עוגיות הדפדפן. הן אינן משתמשות במפתחות API. משתמשים חייבים להיות מחוברים ל-FastComments בדפדפן שלהם כדי לגשת לנקודות קצה אלה.

### Base URL

All Collab Chat API endpoints are under:

[inline-code-attrs-start title = 'כתובת בסיס'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentication

These endpoints authenticate users via browser cookies. They do not use API keys. Users must be logged into FastComments in their browser to access these endpoints.

### Get All Conversations

החזר את כל שיחות הטקסט עבור דף מסוים.

#### Endpoint

[inline-code-attrs-start title = 'נקודת קצה GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

`urlId` (query parameter, required) הוא מזהה הדף שעליו ברצונך לאחזר שיחות.

#### Response

התשובה כוללת את סטטוס ה-API, מידע על סשן המשתמש הנוכחי אם מאומת, מערך שיחות עם ה-IDs שלהן, כתובות URL וטווחי טקסט, מזהה URL מנוקה, דגל המציין האם המשתמש הנוכחי הוא מנהל אתר או מודרטור, ופרטי חיבור WebSocket לסנכרון חי הכוללים את `tenantIdWS`, `urlIdWS`, ו-`userIdWS`.

#### Example Request

[inline-code-attrs-start title = 'דוגמת בקשת GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'דוגמת תגובת GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Create Conversation

צור שיחת טקסט חדשה עבור בחירת טקסט מסוימת.

#### Endpoint

[inline-code-attrs-start title = 'נקודת קצה POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

גוף הבקשה חייב להיות JSON ולכלול את השדות החובה הבאים.

`urlId` (string, required) הוא מזהה הדף הבסיסי.

`urlIdWithRange` (string, required) הוא ה-URL המשולב עם טווח הטקסט, לדוגמה `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) הוא כותרת הדף.

`selector` (string, required) הוא נתיב ה-DOM לאלמנט שמכיל את הטקסט הנבחר.

`range` (string, required) הוא טווח הטקסט הסריאלי בפורמט `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) הוא ה-ID של ההערה שהחלה את הצ'אט הזה.

`broadcastId` (string, required) הוא UUID לסנכרון חי כדי למנוע אפקטי הדהוד.

#### Response

התשובה כוללת את סטטוס ה-API ואת ה-ID של השיחה שנוצרה זה עתה.

#### Example Request

[inline-code-attrs-start title = 'דוגמת בקשת POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'דוגמת תגובת POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Delete Conversation

מחק שיחת טקסט קיימת. נקודת קצה זו דורשת הרשאות מנהל או מודרטור, או שהשיחה חייבת להיות נוצרה על ידי המשתמש המאומת.

#### Endpoint

[inline-code-attrs-start title = 'נקודת קצה DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameters

`tenantId` (path parameter, required) הוא מזהה ה-Tenant של FastComments שלך.

`chatId` (path parameter, required) הוא ה-ID של השיחה למחיקה.

`broadcastId` (request body, required) הוא UUID לסנכרון חי.

#### Example Request

[inline-code-attrs-start title = 'דוגמת בקשת DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'דוגמת תגובת DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Rate Limiting

נקודות קצה אלה כפופות להגבלת קצב הסטנדרטית של FastComments API. ה-middleware של הגבלת הקצב מופעל לכל Tenant כדי למנוע ניצול לרעה תוך שמירה על דפוסי שימוש רגילים.

### Error Responses

כל נקודות הקצה מחזירות קודי סטטוס HTTP סטנדרטיים. תגובת 400 מעידה על פרמטרי בקשה לא חוקיים. תגובת 401 משמעותה שהאימות נכשל. תגובת 403 מציינת חוסר הרשאות מספיקות. תגובת 404 פירושה שהשיחה לא נמצאה. תגובת 429 מעידה על חריגה ממגבלת הקצב.

תגובות שגיאה כוללות גוף JSON עם פרטים:

[inline-code-attrs-start title = 'דוגמת תגובת שגיאה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Usage Tracking

יצירת שיחות מגדילה את מדד השימוש `conversationCreateCount`. כל פעילות סנכרון WebSocket מגדילה את `pubSubMessageCount` ו-`pubSubBandwidth`. ניתן לעקוב אחר מדדים אלה בלוח המחוונים של FastComments תחת ניתוחי שימוש.