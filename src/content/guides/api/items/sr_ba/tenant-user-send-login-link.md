[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Ова рута омогућава слање линка за пријаву једном `TenantUser`-у.

Корисно када се корисници масовно креирају и не морате да им објашњавате како да се пријаве на FastComments.com. Ово ће им једноставно послати „магични линк“ за пријаву који истиче након `30 days`.

Постоје следећа ограничења за слање линка за пријаву `TenantUser`-у:
- `TenantUser` мора већ постојати.
- Морате имати приступ за управљање `Tenant`-ом коме `TenantUser` припада.

Линк за пријаву `TenantUser`-у можемо послати на следећи начин:

[inline-code-attrs-start title = 'Пример cURL захтева за линк за пријаву TenantUser-а'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ово ће послати имејл попут `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Структура захтева за линк за пријаву TenantUser-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за линк за пријаву TenantUser-а'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---