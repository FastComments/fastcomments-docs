[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

יש לאחזר הצבעות לפי `urlId`.

### סוגי הצבעות

ישנם שלושה סוגי הצבעות:

- הצבעות מאומתות, שמיושמות על התגובה המתאימה. אתה יכול ליצור אלה דרך API זה.
- הצבעות מאומתות, שהן **ממתינות** לאימות, ולכן עדיין לא מיושמות על התגובה. אלה נוצרות כאשר משתמש משתמש במנגנון *התחבר כדי להצביע* של FastComments.com.
- הצבעות אנונימיות, שמיושמות על התגובה המתאימה. אלה נוצרות יחד עם תגובות אנונימיות.

אלה מוחזרות ברשימות נפרדות ב-API כדי להפחית בלבול.

[inline-code-attrs-start title = 'דוגמת cURL להצבעות'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת הצבעות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הצבעות'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### הערות הצבעות אנונימיות

שים לב שהצבעות אנונימיות שנוצרו דרך API זה יופיעו ברשימת `appliedAuthorizedVotes`. הן נחשבות מאושרות מכיוון שנוצרו דרך ה-API עם מפתח API.

מבנה `appliedAnonymousVotes` הוא להצבעות שנוצרו ללא אימייל, מפתח API, וכו'.
