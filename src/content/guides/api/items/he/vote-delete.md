[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

נתיב זה מספק את היכולת למחוק `Vote` יחיד.

[inline-code-attrs-start title = 'דוגמת cURL למחיקת הצבעה'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת מחיקת הצבעה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת מחיקת הצבעה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

הערות:

- API זה מציית להגדרות ברמת השוכר. לדוגמה, אם אתה משבית הצבעות לעמוד נתון, וניסית ליצור הצבעה דרך ה-API, זה ייכשל עם קוד שגיאה `voting-disabled`.
- API זה הוא בזמן אמת כברירת מחדל.
- API זה יעדכן את `votes` של `Comment` המתאים.
