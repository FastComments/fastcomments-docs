[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

רשימת ההצבעות הפרטיות מאחורי ספירות של סקר אחד, מהישנה ביותר. קרדיט אחד לכל 100 הצבעות שמוחזרות.

סקר שייך לתגובה, ולכן הצבעות נקראות סקר אחד בכל פעם ו‑`commentId` נדרש. ניתן לצמצם עוד עם `voterId` כדי לבדוק איך אדם אחד הצביע, או עם `optionId` כדי לרשום את כל האנשים שבחרו באפשרות מסוימת.

בכל קריאה מוחזרות לכל היותר 1000 הצבעות. השתמש ב‑`skip` כדי לדפדף לעוד.

הגדרת `privacy` של הסקר מכובדת: הצבעות בסקר אנונימי אינן ניתנות לקריאה, והבקשה נכשלת עם `poll-anonymous`. ראה את המבנה `PollVote` לפרטים.

[inline-code-attrs-start title = 'דוגמת cURL לקבלת PollVotes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת GET ל‑PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת GET ל‑PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### ספירת הצבעות לכל אפשרות

אינך צריך לחבר את אלה כדי לקבל את התוצאות – הסקר מכיל את הספירות שלו. קרא את הסקר עם `GET /api/v1/polls/:commentId` במקום זאת, והשתמש ב‑API זה כאשר אתה צריך לדעת מי הצביע.

### כל סקר בעמוד

אין רשימת הצבעות לכל העמוד. כדי לדווח על עמוד שלם, שלוף את ההערות שלו עם `GET /api/v1/comments`, שמחזיר את הסקר של כל תגובה ואת הספירות שלו, ולאחר מכן קרא את ההצבעות עבור הסקרים שמעניינים אותך.

---