נקודת קצה זו מאפשרת לך לשאוב תגי משתמשים לפי קריטריונים שונים.

דוגמת בקשה:

[inline-code-attrs-start title = 'רשימת תגי משתמשים - דוגמת GET'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X GET "https://fastcomments.com/api/v1/user-badges?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

ניתן להוסיף פרמטרי שאילתה שונים כדי לסנן את התוצאות:

- `userId` - קבל תגים עבור משתמש מסוים
- `badgeId` - קבל מופעים של תג ספציפי
- `type` - סנן לפי סוג תג (0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, וכו'. ראה את מבנה UserBadge לרשימה מלאה)
- `displayedOnComments` - סנן לפי האם התג מוצג על תגובות (true/false)
- `limit` - מספר מקסימלי של תגי להחזיר (ברירת מחדל 30, מקסימום 200)
- `skip` - מספר תגיות לדלג (למטרות עימוד)

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

[inline-code-attrs-start title = 'שגיאה: חסר Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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