[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

רושמת הצבעה בסקר.

לכל מצביע יש הצבעה מרבית אחת לכל סקר. קריאה חוזרת לאותו מצביע מעבירה את הצביעתו לאופציה החדשה במקום להוסיף הצבעה שנייה, והצבעה לאופציה שכבר נבחרה אינה עושה דבר.

התשובה כוללת את הסקר, כך שאתה מקבל את הספירות המעודכנות ללא בקשה נוספת.

[inline-code-attrs-start title = 'דוגמת cURL ליצירת הצבעת סקר'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL ליצירת הצבעת סקר אנונימית'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת הצבעת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** אחד מה‑userId או anonUserId נדרש. **/
    userId?: string
    anonUserId?: string
    /** כתובת ה‑IP של המשתמש הקצה, משמשת למגבלת קצב אנונימית. ברירת המחדל היא ה‑IP של המבצע. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת הצבעת סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** נכלל במקרה של כשל. **/
    reason?: string
    pollVote: PollVote
    /** הסקר עם הספירות המעודכנות שלו. **/
    poll: CommentPoll
}
[inline-code-end]

### הצבעות אנונימיות

הגדר `anonUserId` במקום `userId` כדי לרשום הצבעה עבור מישהו שאינו מחובר. מזהה זה אינו חייב להתאים למשתמש כלשהו - הוא רק מזהה את המושב, ולכן אותו אדם אינו נספר פעמיים.

הצבעה אנונימית חייבת להיות מופעלת באתר שלך. אם ההצבעה מוגבלת למשתמשים מחוברים, הצבעה עם `anonUserId` בלבד תיכשל עם `poll-login-required`.

הצבעות אנונימיות גם מוגבלות בקצב לפי IP לכל סקר, כדי למנוע מאדם אחד למלא סקר על‑ידי ניקוי המושב שלו. שלח את ה‑`ip` של המשתמש הקצה כך שהמגבלה תחול עליו ולא על השרת שלך.

### הערות נוספות

- `userId` חייב להיות של משתמש קיים באתר שלך. הצבעות למשתמש השייך לאתר אחר נדחות.
- הצבעה בסקר סגור תיכשל עם `poll-closed`.
- API זה מעדכן את הספירות בסקר ודוחף אותן לוידג'טים המחוברים בזמן אמת.