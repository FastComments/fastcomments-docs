[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Fjerner en af den aktuelle brugers reaktioner fra en side. Hvis brugeren ikke har tilføjet den reaktion, lykkes anmodningen med koden `no-react` og ændrer ikke tælleren.

[inline-code-attrs-start title = 'Eksempel på Page React Delete cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Delete anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Reaktions-id. **/
    id: string
    /** URI-kodet JSON af dit SSO-objekt. Udelad for anonyme brugere. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Delete svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' når brugeren ikke havde tilføjet denne reaktion. Ellers inkluderet ved fejl. **/
    code?: 'no-react' | string
    /** Inkluderet ved fejl. **/
    reason?: string
}
[inline-code-end]