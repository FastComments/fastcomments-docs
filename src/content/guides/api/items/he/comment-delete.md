---
[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה זו של ה-API מאפשרת למחוק תגובה.

הערות:

- API זה יכול לעדכן את ווידג'ט התגובות "בלייב" אם רצוי (זה מעלה את `creditsCost` מ-`1` ל-`2`).
- ה-API ימחק את כל תגובות המשנה.
- אם התגובה המיועדת נעולה (`isLocked: true`), הבקשה תידחה עם `code: 'locked'`. הסר את הנעילה של התגובה תחילה, ואז מחק אותה.

[inline-code-attrs-start title = 'דוגמת cURL למחיקת תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'מבנה בקשת DELETE לתגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** המשתמש שמבצע את העדכון. במידת הצורך ניתן להשתמש בו כדי לבדוק שהוא יכול למחוק את התגובה.  **/
    contextUserId?: string
	/** האם יש למחוק את התגובה "בלייב" עבור משתמשים הצופים במופעים של ווידג'ט התגובות עם אותו urlId. שימו לב: מכפיל את עלות האשראי מ-1 ל-2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מחיקת תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כישלון. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** כלול במקרה של כישלון. **/
    reason?: string
}
[inline-code-end]

---