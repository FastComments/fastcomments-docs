[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

מאפשר אחזור הצבעות שהושארו על ידי משתמש ב-`urlId` נתון. מקבל `userId` שיכול להיות כל משתמש FastComments.com או `SSO User`.

זה שימושי אם אתה רוצה להראות אם משתמש הצביע על תגובה. בעת אחזור תגובות, פשוט קרא ל-API זה באותו זמן עבור המשתמש עם
אותו `urlId`.

אם אתה משתמש בהצבעה אנונימית אז תרצה להעביר `anonUserId` במקום.

[inline-code-attrs-start title = 'דוגמת cURL להצבעות עבור משתמש'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL להצבעות עבור משתמש אנונימי'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

שים לב שהצבעות אנונימיות יופיעו ברשימת `appliedAuthorizedVotes`. הן נחשבות מורשות מכיוון שנוצרו דרך ה-API עם מפתח API.

[inline-code-attrs-start title = 'מבנה בקשת הצבעות עבור משתמש'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הצבעות עבור משתמש'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
