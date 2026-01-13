[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לעדכן תגובה בודדת.

הערות:

- API זה יכול לעדכן את ווידג'ט התגובות "בזמן אמת" אם רצוי (זה מעלה את בסיס `creditsCost` מ-`1` ל-`2`).
  - זה יכול להפוך העברת תגובות בין דפים ל"בזמן אמת" (שינוי `urlId`).
  - העברות עולות `2` קרדיטים נוספים מכיוון שדפים מחושבים מראש וזה צורך מעבד.
- בניגוד ל-API היצירה, API זה לא ייצור אוטומטית אובייקטי משתמש במערכת שלנו אם אימייל מסופק.
- תגובות שעודכנו דרך API זה עדיין יכולות להיבדק לספאם אם רצוי.
- קונפיגורציה כגון אורך תגובה מקסימלי, אם מוגדרת דרך דף ניהול כללי ההתאמה האישית, תחול כאן.
- כדי לאפשר למשתמשים לעדכן את טקסט התגובה שלהם, אתה יכול פשוט לציין `comment` בגוף הבקשה. אנחנו ניצור את `commentHTML` המתקבל.
  - אם אתה מגדיר גם `comment` וגם `commentHTML` לא ניצור אוטומטית את ה-HTML.
  - אם המשתמש מוסיף אזכורים או האשטאגים בטקסט החדש שלו, הוא עדיין יעובד כמו ה-API של `POST`.
- בעת עדכון `commenterEmail` על תגובה, עדיף לציין גם `userId`. אחרת, עליך לוודא שהמשתמש עם אימייל זה שייך לשוכר שלך, או שהבקשה תיכשל.


[inline-code-attrs-start title = 'דוגמת cURL מינימלית ל-PATCH תגובה'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת PATCH תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
