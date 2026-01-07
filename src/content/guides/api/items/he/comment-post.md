[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת ליצור תגובות.

מקרי שימוש נפוצים הם ממשקי משתמש מותאמים אישית, אינטגרציות או ייבוא.

הערות:

- API זה יכול לעדכן את ווידג'ט התגובות "בזמן אמת" אם רצוי (זה מעלה את `creditsCost` מ-`1` ל-`2`).
- API זה ייצור אוטומטית אובייקטי משתמש במערכת שלנו אם אימייל מסופק.
- ניסיון לשמור שתי תגובות עם אימיילים שונים, אבל אותו שם משתמש, יגרום לשגיאה עבור התגובה השנייה.
- אם אתה מציין `parentId`, ולתגובת ילד יש `notificationSentForParent` כ-false, **נשלח התראות עבור תגובת ההורה**. זה נעשה כל שעה (אנחנו מאגדים את ההתראות יחד כדי להפחית את מספר האימיילים הנשלחים).
- אם אתה רוצה לשלוח אימיילי ברכה בעת יצירת משתמשים, או אימיילי אימות תגובה, הגדר `sendEmails` ל-`true` בפרמטרי השאילתה.
- תגובות שנוצרו דרך API זה יופיעו בדפי האנליטיקה והמודרציה של אפליקציית הניהול.
- "מילים רעות" עדיין מוסתרות בשמות המגיבים ובטקסט התגובות אם ההגדרה מופעלת.
- תגובות שנוצרו דרך API זה עדיין יכולות להיבדק לספאם אם רצוי.
- קונפיגורציה כגון אורך תגובה מקסימלי, אם מוגדרת דרך דף ניהול כללי ההתאמה האישית, תחול כאן.

הנתונים המינימליים הנדרשים להגשה שיוצגו בווידג'ט התגובות הם כדלקמן:

[inline-code-attrs-start title = 'דוגמת cURL מינימלית ל-POST תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

בקשה יותר ריאליסטית עשויה להיראות כך:

[inline-code-attrs-start title = 'דוגמת cURL ל-POST תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת POST תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת POST תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
