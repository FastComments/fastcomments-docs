[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת למחוק תגובה.

הערות:

- API זה יכול לעדכן את ווידג'ט התגובות "בזמן אמת" אם רצוי (זה מעלה את `creditsCost` מ-`1` ל-`2`).
- API זה ימחק את כל תגובות הילדים.

[inline-code-attrs-start title = 'דוגמת cURL למחיקת תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'מבנה בקשת מחיקת תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can delete the comment.  **/
    contextUserId?: string
	/** Whether the comment should be deleted "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מחיקת תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
