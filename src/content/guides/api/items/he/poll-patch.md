[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

עורך סקר מבלי להפריע להצבעות שלו. השתמש בזה כדי לתקן שגיאת כתיב בשאלה או באפשרות, כדי לסגור או
לפתוח מחדש את הסקר, או כדי לשנות מי יכול לראות מי הצביע.

האפשרויות מזוהות על ידי ה-`id` שלהן, ו-`PATCH` משנה את התוויות של אלו שאתה מציין. כדי להוסיף, להסיר או לשנות סדר
של אפשרויות, שלח את רשימת האפשרויות המלאה ל-`PUT /api/v1/polls/:commentId`: האפשרויות שאתה שולח עם ה-id שלהם שומרות
גם את ההצבעות שלהן.

כל שדה הוא אופציונלי, אך לפחות אחד חייב להינתן.

[inline-code-attrs-start title = 'דוגמת cURL לתיקון סקר'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL לסגירת סקר עכשיו'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת תיקון סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת תיקון סקר'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### הערות נוספות

- מתן שם ל-id של אפשרות שאינה קיימת בסקר גורם לשגיאה `poll-invalid` במקום לשתוק ולהתעלם.
- התוויות חייבות להישאר ייחודיות בתוך הסקר, כולל האפשרויות שאינך משנה.
- בשונה מיצירת סקר, `closesAt` יכול להיות בעבר כאן - כך אתה סוגר סקר מיידית.
- פרטיות הסקר יכולה להיות מצומצמת אך לא מורחבת לאחר שיש הצבעות.
- הערה נעולה לא יכולה לשנות את הסקר שלה, והפעולה תיכשל עם `locked`.

---