 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת להוסיף `Vote` מאושר יחיד. הצבעות יכולות להיות `up` (+1) או `down` (-1).

[inline-code-attrs-start title = 'דוגמת cURL ליצירת הצבעה'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL ליצירת הצבעה אנונימית'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת יצירת הצבעה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת יצירת הצבעה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### יצירת הצבעות אנונימיות

ניתן ליצור הצבעות אנונימיות על ידי הגדרת `anonUserId` בפרמטרי השאילתה במקום `userId`.

מזהה זה לא חייב להתאים לאובייקט משתמש בשום מקום (ומכאן אנונימי). זה פשוט מזהה
עבור הסשן, כך שתוכל לאחזר הצבעות שוב באותו סשן, כדי לבדוק אם הצביעו על תגובה.

אם אין לך דבר כזה כמו "סשנים אנונימיים" כמו ל-FastComments - אתה יכול פשוט
להגדיר זאת למזהה אקראי, כמו UUID (אם כי אנחנו מעריכים מזהים קטנים יותר לחיסכון במקום).

### הערות נוספות

- API זה מציית להגדרות ברמת השוכר. לדוגמה, אם אתה משבית הצבעות לעמוד נתון, וניסית ליצור הצבעה דרך ה-API, זה ייכשל עם קוד שגיאה `voting-disabled`.
- API זה הוא בזמן אמת כברירת מחדל.
- API זה יעדכן את `votes` של `Comment` המתאים.
