נקודת קצה זו מאפשרת לך לאחזר תגי משתמש על בסיס קריטריונים שונים.

דוגמת בקשה:

[inline-code-attrs-start title = 'דוגמת בקשת GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

אתה יכול להוסיף פרמטרי שאילתה שונים כדי לסנן את התוצאות:

- `userId` - קבל תגים למשתמש ספציפי
- `badgeId` - קבל מופעים של תג ספציפי
- `type` - סנן לפי סוג תג (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, וכו'. ראה מבנה UserBadge לרשימה מלאה)
- `displayedOnComments` - סנן לפי האם התג מוצג בתגובות (true/false)
- `limit` - מספר מקסימלי של תגים להחזרה (ברירת מחדל 30, מקסימום 200)
- `skip` - מספר תגים לדלג עליהם (לעימוד)

דוגמת תגובה:

[inline-code-attrs-start title = 'תגובה'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "userBadges": [
    {
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
    },
    {
      "id": "badge124",
      "userId": "user456",
      "badgeId": "badgeDef790",
      "fromTenantId": "tenant001",
      "createdAt": 1650532598000,
      "receivedAt": 1650532598000,
      "type": 0,
      "threshold": 100,
      "name": "Centurion",
      "description": "Made 100 comments",
      "displayLabel": "100",
      "backgroundColor": "#2b6cb0",
      "textColor": "#ffffff",
      "displayedOnComments": true,
      "order": 2
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
