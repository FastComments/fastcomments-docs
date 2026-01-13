נקודת קצה זו מאפשרת לך לאחזר רשומת התקדמות תג של משתמש לפי מזהה המשתמש שלו.

דוגמת בקשה:

[inline-code-attrs-start title = 'דוגמת בקשת GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress/user/user456?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

דוגמת תגובה:

[inline-code-attrs-start title = 'תגובה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgress": {
    "id": "progress123",
    "tenantId": "tenant001",
    "userId": "user456",
    "firstCommentId": "comment789",
    "firstCommentDate": 1650532511000,
    "autoTrustFactor": 0.75,
    "progress": {
      "0": 42,
      "1": 120,
      "2": 15,
      "3": 3,
      "5": 5,
      "6": 1800000,
      "8": 0,
      "7": 0
    }
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
  "reason": "The User ID (path param: userId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'שגיאה: לא נמצא'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "No badge progress found for user user456 in tenant tenant001."
}
[inline-code-end]
