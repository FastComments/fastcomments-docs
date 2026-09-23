[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

מצרף סקר לתגובה קיימת, או מגדיר את המצב המלא של הסקר שכבר קיים.

הגוף הוא הסקר המלא, והאפשרויות שאתה שולח הופכות לאפשרויות של הסקר, באותו סדר. כל אפשרות מתואמת לפי ה-`id` שלה:

- אפשרות שנשלחת עם ה-`id` של אפשרות קיימת משמרת את האפשרות ואת ההצבעות שלה. התווית והמיקום שלה מתעדכנים למה ששלחת.  
- אפשרות שנשלחת ללא `id` מתווספת, ללא הצבעות.  
- אפשרות קיימת שאתה משמיט מוסרת, יחד עם ההצבעות שהתקבלו עליה. `totalVotes` יורד באותו הסכום.

לכן, כדי להוסיף אפשרות, שלח את האפשרויות הנוכחיות עם המזהים שלהן יחד עם החדשה ללא מזהה. כדי להסיר אפשרות, שלח את הרשימה ללא אותה אפשרות. מזהי האפשרויות נמצאים בסקר שמוחזר על ידי `GET /api/v1/polls/:commentId`.

שליחת כלום ללא מזהים מחליפה כל אפשרות ומוחקת כל הצבעה שכבר נרשמה בסקר. אם לסקר יש הצבעות, זה דורש `replaceVotes=true`, וללא זאת ה-API מחזיר `replace-votes-required`.

השדות האחרים גם מוחלפים: השמטת `closesAt`, `privacy` או `requireVoteToSeeResults` מאפסת אותם לברירת המחדל. כדי לשנות שדה יחיד ולהשאיר את השאר ללא שינוי, השתמש ב-`PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'דוגמת cURL של Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** נדרש כדי לשמור על אף אחד ממזהי האפשרויות הקיימות כאשר לסקר יש הצבעות, מכיוון שזה מוחק את כולם. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** המזהה של אפשרות קיימת, כדי לשמור עליה ועל ההצבעות שלה. השמטה מוסיפה אפשרות חדשה. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** הרשימה המלאה והמסודרת. אפשרויות קיימות שהושמטו יוסרו יחד עם ההצבעות שלהן. **/
    options: PollPutOption[]
    /** חייבת להיות בעתיד כאשר לתגובה עדיין אין סקר. השמטה עבור סקר שנשאר פתוח. **/
    closesAt?: string | null
    /** 0 אנונימי (ברירת מחדל), 1 מנהלים ומפקחים, 2 כולם. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** כלול במקרה של כשל. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** כלול במקרה של כשל. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### הערות נוספות

- `id` שלא קיים בסקר, או `id` זהה שניתן פעמיים, גורם לכשל עם `poll-invalid`. תגובה ללא סקר עדיין אין לה מזהי אפשרויות, ולכן כל אפשרות שנשלחת אליה חייבת להשמיט את `id`.  
- פרטיות הסקר יכולה להיות מצומצמת אך לא מורחבת לאחר שיש לה הצבעות.  
- ה-API הזה מציית להגדרות האתר שלך. אם סקרים אינם מופעלים עבור האתר או העמוד, הוא נכשל עם `polls-disabled`.  
- תגובה נעולה אינה יכולה לשנות את הסקר שלה, והפעולה נכשלת עם `locked`.  
- ווידג'טים מחוברים מתעדכנים בזמן אמת, כך שהצופים רואים את הסקר החדש ללא צורך ברענון.