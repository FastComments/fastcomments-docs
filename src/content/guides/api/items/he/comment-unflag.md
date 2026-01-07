[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לבטל דיווח על תגובה עבור משתמש ספציפי.

הערות:

- קריאה זו חייבת תמיד להיעשות בהקשר של משתמש. המשתמש יכול להיות משתמש FastComments.com, משתמש SSO או משתמש שוכר.
- לאחר שתגובה מאושרת אוטומטית (מוסתרת) - התגובה יכולה להיות מאושרת מחדש רק על ידי מנהל או מנחה. ביטול הדיווח לא יאשר מחדש את התגובה.

[inline-code-attrs-start title = 'דוגמת cURL לביטול דיווח על תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

לדיווח אנונימי, עלינו לציין `anonUserId`. זה יכול להיות מזהה שמייצג את ההפעלה האנונימית, או UUID אקראי.

[inline-code-attrs-start title = 'דוגמת cURL לדיווח אנונימי על תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'מבנה בקשת ביטול דיווח על תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת ביטול דיווח על תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
