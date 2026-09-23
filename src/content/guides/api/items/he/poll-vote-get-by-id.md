[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

קורא הצבעה בודדת בסקר לפי המזהה שלה.

הצבעה בסקר אנונימי אינה ניתנת לקריאה, והבקשה נכשלת עם `poll-anonymous`. ראו את המבנה `PollVote` כדי להבין כיצד הגדרת `privacy` של הסקר חלה.

[inline-code-attrs-start title = 'דוגמת cURL לקבלת PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת קבלת PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת קבלת PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כישלון. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** נכלל במקרה של כישלון. **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]