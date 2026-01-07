נקודת קצה זו מאפשרת לך ליצור הקצאת תג משתמש חדשה.

דוגמת בקשה:

[inline-code-attrs-start title = 'דוגמת בקשת POST'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "userId": "user456",
  "badgeId": "badgeDef789",
  "displayedOnComments": true
}'
[inline-code-end]

גוף הבקשה חייב להכיל את הפרמטרים הבאים:

- `userId` (נדרש) - מזהה המשתמש להקצאת התג אליו
- `badgeId` (נדרש) - מזהה התג להקצאה
- `displayedOnComments` (אופציונלי) - האם התג יוצג בתגובות המשתמש (ברירת מחדל true)

הערות חשובות:
1. התג חייב להתקיים ולהיות מופעל בקטלוג התגים של השוכר שלך
2. אתה יכול להקצות תגים רק למשתמשים השייכים לשוכר שלך או שהגיבו באתר שלך

דוגמת תגובה:

[inline-code-attrs-start title = 'תגובה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadge": {
    "id": "badge123",
    "userId": "user456",
    "badgeId": "badgeDef789",
    "fromTenantId": "tenant001",
    "createdAt": 1650532511000,
    "receivedAt": 1650532511000,
    "type": 14,
    "name": "Special Contributor",
    "description": "Awarded to special contributors to our community",
    "displayLabel": "Special",
    "backgroundColor": "#4a5568",
    "textColor": "#ffffff",
    "displayedOnComments": true,
    "order": 1
  }
}
[inline-code-end]

תגובות שגיאה אפשריות:

[inline-code-attrs-start title = 'שגיאה: מזהה שוכר חסר'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'שגיאה: מזהה משתמש חסר'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-user-id",
  "reason": "User ID (body param: userId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'שגיאה: מזהה תג חסר'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-badge-id",
  "reason": "Badge ID (body param: badgeId) is required."
}
[inline-code-end]

[inline-code-attrs-start title = 'שגיאה: תג לא נמצא'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "badge-not-found",
  "reason": "The badge badgeDef789 was not found or is not enabled."
}
[inline-code-end]

[inline-code-attrs-start title = 'שגיאה: משתמש לא מורשה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "unauthorized-user",
  "reason": "You can only add badges to users who belong to your tenant or have commented on your site."
}
[inline-code-end]
