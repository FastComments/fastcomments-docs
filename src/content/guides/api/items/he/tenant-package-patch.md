[api-resource-header-start name = 'TenantPackage'; route = 'PATCH /api/v1/tenant-packages/:id'; creditsCost = 1; api-resource-header-end]

נקודת קצה API זו מספקת את היכולת לעדכן `TenantPackage` לפי `id`.

לעדכון `TenantPackage` יש את ההגבלות הבאות:

- אם אתה מגדיר `hasFlexPricing` ל-true, אז כל פרמטרי ה-`flex*` נדרשים באותה בקשה.
- ה-`name` לא יכול להיות ארוך מ-`50 תווים`.
- כל פריט `forWhoText` לא יכול להיות ארוך מ-`200 תווים`.
- כל פריט `featureTaglines` לא יכול להיות ארוך מ-`100 תווים`.
- ה-`TenantPackage` חייב להיות "קטן יותר" מהשוכר האב. לדוגמה, כל פרמטרי ה-`max*` חייבים להיות בעלי ערכים נמוכים יותר מהשוכר האב.
- לא ניתן לשנות את ה-`tenantId` המשויך ל-`TenantPackage`.

[inline-code-attrs-start title = 'דוגמת cURL ל-PATCH TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת PATCH TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface TenantPackagePatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
