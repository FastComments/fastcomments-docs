[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה זו של ה-API מספקת את היכולת לעדכן תגובה אחת.

הערות:

- API זה יכול לעדכן את ווידג'ט התגובות "בזמן אמת" אם רצוי (זה מגדיל את ה-`creditsCost` הבסיסי מ-`1` ל-`2`).
  - זה יכול לגרום להגירת תגובות בין עמודים להיות "בזמן אמת" (שינוי של `urlId`).
  - הגירות עולות בנוסף `2` קרדיטים כיוון שהעמודים מחושבים מראש וזאת פעולה עתירת CPU.
- בניגוד ל-API של יצירה, API זה לא יווצר אוטומטית אובייקטי משתמש במערכת שלנו אם מסופק אימייל.
- תגובות שמעודכנות דרך API זה עדיין יכולות להיבדק כספאם אם רצוי.
- הגדרות כגון אורכו המרבי של תגובה, אם הוגדרו דרך דף הניהול של כלל ההתאמה (Customization Rule), יחולו כאן.
- כדי לאפשר למשתמשים לעדכן את טקסט התגובה שלהם, ניתן פשוט לציין את `comment` בגוף הבקשה. אנו נייצר את ה-`commentHTML` הנובע.
  - אם תגדיר גם `comment` וגם `commentHTML` לא נייצר את ה-HTML באופן אוטומטי.
  - אם המשתמש יוסיף אזכורים או האשטגים בטקסט החדש שלו, הם עדיין יעובדו כמו ב-API ה-`POST`.
- בעת עדכון `commenterEmail` על תגובה, עדיף גם לציין `userId`. אחרת, עליך להבטיח שהמשתמש עם אימייל זה שייך ל-tenant שלך, אחרת הבקשה תיכשל.  
- אם התגובה היעד נעולה (`isLocked: true`), הבקשה תידחה עם `code: 'locked'`. פתח את התגובה קודם, עדכן אותה, ואז נעל שוב אם רצוי.


[inline-code-attrs-start title = 'דוגמת cURL מינימלית ל-PATCH של תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH לתגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** המשתמש שעושה את העדכון. במידת הצורך ניתן להשתמש בזה כדי לבדוק שהוא יכול לערוך את התגובה.  **/
    contextUserId?: string
	/** האם יש לבדוק אם התגובה החדשה נראית כספאם?  **/
    doSpamCheck?: 'true' | 'false'
	/** האם התגובה תופיע "בזמן אמת" למשתמשים הצופים במופעים של ווידג'ט התגובות עם אותו urlId. הערה: מכפיל את עלות הקרדיטים מ-1 ל-2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תשובת PATCH לתגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** נכלל במקרה של כישלון. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** נכלל במקרה של כישלון. **/
    reason?: string
}
[inline-code-end]