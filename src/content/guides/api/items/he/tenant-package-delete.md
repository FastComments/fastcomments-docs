[api-resource-header-start name = 'TenantPackage'; route = 'DELETE /api/v1/tenant-packages/:id'; creditsCost = 5; api-resource-header-end]

נתיב זה מספק הסרה של `TenantPackage` לפי מזהה.

לא ניתן להסיר `TenantPackage` שנמצא בשימוש (ה-`packageId` של שוכר מצביע על החבילה). עדכן את ה-`Tenant` קודם.

[inline-code-attrs-start title = 'דוגמת cURL להסרת TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-packages/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'מבנה בקשת הסרת TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'מבנה תגובת הסרת TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackageDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized' | 'package-in-use'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
