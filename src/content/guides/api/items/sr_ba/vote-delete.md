[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава брисање појединачног `Vote`.

[inline-code-attrs-start title = 'Примјер cURL захтјева за брисање гласа'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за брисање гласа'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање гласа'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]

Напомене:

- Овај API се придржава подешавања на нивоу тенанта. На примјер, ако онемогућите гласање за одређену страницу, и покушате кроз API да креирате глас, он ће неуспјети са кодом грешке `voting-disabled`.
- Овај API је по подразумјеваном активан.
- Овај API ће ажурирати `votes` одговарајућег `Comment`.

---