נקודת קצה זו מאפשרת לך לאחזר רשומות התקדמות תג משתמש על בסיס קריטריונים שונים.

דוגמת בקשה:

[inline-code-attrs-start title = 'דוגמת בקשת GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badge-progress?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

אתה יכול להוסיף פרמטרי שאילתה שונים כדי לסנן את התוצאות:

- `userId` - קבל התקדמות למשתמש ספציפי
- `limit` - מספר מקסימלי של רשומות להחזרה (ברירת מחדל 30, מקסימום 200)
- `skip` - מספר רשומות לדלג עליהן (לעימוד)

דוגמת תגובה:

[inline-code-attrs-start title = 'תגובה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadgeProgresses": [
    {
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
    },
    {
      "id": "progress124",
      "tenantId": "tenant001",
      "userId": "user789",
      "firstCommentId": "comment790",
      "firstCommentDate": 1650532598000,
      "autoTrustFactor": 0.5,
      "progress": {
        "0": 12,
        "1": 15,
        "2": 4
      }
    }
  ]
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

[inline-code-attrs-start title = 'שגיאה: limit לא חוקי'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "invalid-limit",
  "reason": "The limit (query param: limit) is too large (> 200)."
}
[inline-code-end]
