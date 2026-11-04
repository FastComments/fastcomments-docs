### סקירת API

Collab Chat מספק שלושה נקודות קצה (endpoints) של REST API לניהול שיחות טקסט באופן תוכנתי. נקודות קצה אלו מאפשרות לך לאחזר, ליצור ולמחוק הערות (annotations) מבלי להשתמש בווידג'ט הדפדפן.

אלו נקודות קצה ציבוריות שמאמתות משתמשים באמצעות עוגיות דפדפן. הן אינן משתמשות במפתחות API. המשתמשים חייבים להיות מחוברים ל-FastComments בדפדפן שלהם כדי לגשת לנקודות קצה אלו.

### כתובת בסיס (Base URL)

כל נקודות הקצה של Collab Chat API נמצאות תחת:

[inline-code-attrs-start title = 'כתובת בסיס'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### אימות

נקודות קצה אלו מאמתות משתמשים באמצעות עוגיות דפדפן. הן אינן משתמשות במפתחות API. המשתמשים חייבים להיות מחוברים ל-FastComments בדפדפן שלהם כדי לגשת לנקודות קצה אלו.

### קבלת כל השיחות

אחזר את כל שיחות הטקסט עבור דף ספציפי.

#### נקודת קצה

[inline-code-attrs-start title = 'נקודת קצה GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

`urlId` (פרמטר שאילתה, נדרש) הוא מזהה הדף שעבורו ברצונך לאחזר שיחות.

#### תגובה

התגובה כוללת את סטטוס ה-API, מידע על סשן המשתמש הנוכחי אם האימות בוצע, מערך של שיחות עם המזהים, הכתובות וטווחי הטקסט שלהן, מזהה כתובת מנוקה, דגל המציין אם המשתמש הנוכחי הוא מנהל אתר או מנחה (moderator), ופרטי חיבור WebSocket לסנכרון חי כולל `tenantIdWS`, `urlIdWS` ו-`userIdWS`.

#### דוגמת בקשה

[inline-code-attrs-start title = 'דוגמת בקשת GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### דוגמת תגובה

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

### יצירת שיחה

צור שיחת טקסט חדשה עבור בחירת טקסט ספציפית.

#### נקודת קצה

[inline-code-attrs-start title = 'נקודת קצה POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

גוף הבקשה חייב להיות JSON ולכלול את השדות הנדרשים הבאים.

`urlId` (מחרוזת, נדרש) הוא מזהה הדף הבסיסי.

`urlIdWithRange` (מחרוזת, נדרש) הוא הכתובת המשולבת עם טווח הטקסט, לדוגמה `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (מחרוזת, נדרש) הוא כותרת הדף.

`selector` (מחרוזת, נדרש) הוא הנתיב ב-DOM לאלמנט המכיל את הטקסט הנבחר.

`range` (מחרוזת, נדרש) הוא טווח הטקסט המסודר בפורמט `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (מחרוזת, נדרש) הוא המזהה של התגובה שיזמה שיחה זו.

`broadcastId` (מחרוזת, נדרש) הוא UUID לסנכרון חי למניעת אפקטי הדהוד (echo).

#### תגובה

התגובה כוללת את סטטוס ה-API ואת המזהה של השיחה החדשה שנוצרה.

#### דוגמת בקשה

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

#### דוגמת תגובה

[inline-code-attrs-start title = 'דוגמת תגובת POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### מחיקת שיחה

מחק שיחת טקסט קיימת. נקודת קצה זו דורשת הרשאות מנהל או מנחה (moderator), או שהשיחה חייבת להיות נוצרה על ידי המשתמש המאומת.

#### נקודת קצה

[inline-code-attrs-start title = 'נקודת קצה DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### פרמטרים

`tenantId` (פרמטר נתיב, נדרש) הוא מזהה ה-Tenant שלך ב-FastComments.

`chatId` (פרמטר נתיב, נדרש) הוא המזהה של השיחה שיש למחוק.

`broadcastId` (גוף הבקשה, נדרש) הוא UUID לסנכרון חי.

#### דוגמת בקשה

[inline-code-attrs-start title = 'דוגמת בקשת DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### דוגמת תגובה

[inline-code-attrs-start title = 'דוגמת תגובת DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### הגבלת קצב (Rate Limiting)

נקודות קצה אלו כפופות להגבלת הקצב הסטנדרטית של FastComments API. תוכנת הביניים (middleware) להגבלת קצב חלה פר-Tenant כדי למנוע שימוש לרעה תוך אפשור דפוסי שימוש תקינים.

### תגובות שגיאה

כל נקודות הקצה מחזירות קודי סטטוס HTTP סטנדרטיים. תגובת 400 מציינת פרמטרי בקשה לא תקינים. תגובת 401 פירושה שהאימות נכשל. תגובת 403 מציינת הרשאות לא מספיקות. תגובת 404 פירושה שהשיחה לא נמצאה. תגובת 429 מציינת שהוגבלת הקצב.

תגובות שגיאה כוללות גוף JSON עם פרטים:

[inline-code-attrs-start title = 'דוגמת תגובת שגיאה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### מעקב שימוש

יצירת שיחות מגדילה את מדד השימוש `conversationCreateCount` שלך. כל פעילות סנכרון WebSocket מגדילה את `pubSubMessageCount` ו-`pubSubBandwidth`. באפשרותך לעקוב אחר מדדים אלו בלוח הבקרה שלך ב-FastComments תחת ניתוח שימוש.