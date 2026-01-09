[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава брисање појединачног `Vote`.

[inline-code-attrs-start title = 'Пример cURL захтева за брисање Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за брисање Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

Напомене:

- Овај API поштује подешавања на нивоу tenant-а. На пример, ако онемогућите гласање за одређену страницу, и покушате да преко API-ја креирате глас, то ће пропасти са кодом грешке `voting-disabled`.
- Овај API је по подразумеву активан.
- Овај API ће ажурирати `votes` одговарајућег `Comment`.