[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

נתיב זה מספק הסרה של `TenantUser` לפי מזהה.

מחיקת התגובות של המשתמש אפשרית באמצעות פרמטר השאילתה `deleteComments`. שים לב שאם זה true:

1. כל התגובות של המשתמש יימחקו בזמן אמת.
2. כל התגובות __ילדים__ (עכשיו יתומות) יימחקו או יאונמו בהתבסס על קונפיגורציית העמוד המשויכת לכל תגובה. לדוגמה אם מצב מחיקת שרשור הוא "anonymize", אז תשובות יישארו, ותגובות המשתמש יאונמו. זה חל רק כאשר `commentDeleteMode` הוא `Remove` (ערך ברירת המחדל).
3. ה-`creditsCost` הופך ל-`2`.

### תגובות מאונמות

אתה יכול לשמור את התגובות של המשתמש אבל פשוט לאנונם אותן על ידי הגדרת `commentDeleteMode=1`.

אם התגובות של המשתמש מאונמות אז הערכים הבאים מוגדרים ל-null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` ו-`isDeletedUser` מוגדרים ל-`true`.

בעת רינדור, ווידג'ט התגובות ישתמש ב-`DELETED_USER_PLACEHOLDER` (ברירת מחדל: "[deleted]") לשם המשתמש ו-`DELETED_CONTENT_PLACEHOLDER` לתגובה. ניתן להתאים אישית אלה דרך ממשק המשתמש של התאמת הווידג'ט.

### דוגמאות

[inline-code-attrs-start title = 'דוגמת cURL להסרת TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת הסרת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הסרת TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
