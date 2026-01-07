[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at slette en enkelt `Vote`.

[inline-code-attrs-start title = 'Vote Sletning cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Sletning Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Sletning Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

Bemærkninger:

- Dette API overholder tenant-niveau indstillinger. For eksempel, hvis du deaktiverer afstemning for en given side, og du forsøger at oprette en stemme via API'et, vil det fejle med fejlkode `voting-disabled`.
- Dette API er live som standard.
- Dette API vil opdatere `votes` for den tilsvarende `Comment`.
