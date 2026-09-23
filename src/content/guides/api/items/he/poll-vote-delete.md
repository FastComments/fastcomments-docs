[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

מבטל הצבעה. האפשרות שעליה הוצבעה ההצבעה מחזירה את הספירה שלה, והמצביע חופשי להצביע שוב.

[inline-code-attrs-start title = 'דוגמת cURL למחיקת הצבעת סקר'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת מחיקת הצבעת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מחיקת הצבעת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### הערות נוספות

- מחיקת אותה הצבעה פעמיים מחזירה `not-found` בפעם השנייה, והספירות נשארות ללא שינוי.
- אם הסקר הוחלף מאז שההצבעה נרשמה, ההצבעה מוסרת אך אין שינוי בספירות, מכיוון שההחלפה התחילה מאפס.