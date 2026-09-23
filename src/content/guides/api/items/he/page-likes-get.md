[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

מחזיר את מספר הלייקים בעמוד, והאם המשתמש הנוכחי אהב אותו. דפים שעדיין אינם קיימים מחזירים `likeCount` של `0`.

[inline-code-attrs-start title = 'דוגמת cURL של לייקים בעמוד'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** JSON מקודד ב-URI של אובייקט ה-SSO שלך. השמט עבור משתמשים אנונימיים. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת לייקים בעמוד'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כשל. **/
    code?: string
    /** כלול במקרה של כשל. **/
    reason?: string
    likeCount: number
    /** האם המשתמש המבצע את הבקשה אהב את העמוד. **/
    didLike: boolean
    /** מספר ההערות ברמה העליונה בעמוד. **/
    commentCount: number
    /** המזהה המשמש למנוי לעדכונים בזמן אמת עבור דף זה. **/
    urlIdWS: string
}
[inline-code-end]