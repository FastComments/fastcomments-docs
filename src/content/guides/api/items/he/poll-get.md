[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

קורא את הסקר המצורף לתגובה, עם ספירות ההצבעה הנוכחיות.

סקרים גם מוחזרים בתגובה עצמה על ידי ה-APIs של תגובות, ולכן השתמשו בזה כאשר אתם רוצים רק את התוצאות ולא את כל התגובה.

[inline-code-attrs-start title = 'דוגמת cURL לקבלת סקר'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת קבלת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת קבלת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

תגובה שאין לה סקר, תגובה שנמחקה, ומזהה תגובה שאינו קיים, כולם מחזירים את אותה תגובה, עם `poll-not-found`.