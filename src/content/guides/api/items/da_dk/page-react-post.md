---
[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Tilføjer en reaktion til en side som den aktuelle bruger. En bruger kan tilføje hver reaktions-id én gang: at tilføje den igen lykkes med koden `already-reacted` og ændrer ikke tælleren. En bruger kan tilføje flere forskellige reaktioner til den samme side.

Reaktions-id'er vælges af dig og kan være op til 36 tegn. Siden oprettes, hvis den endnu ikke findes. Send `title` for at angive eller opdatere sidens titel.

[inline-code-attrs-start title = 'Side Reaktion cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Side Reaktion Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** Reaktions-id'et, op til 36 tegn. **/
    id: string
    /** Angiver sidens titel. **/
    title?: string
    /** URI-kodet JSON af dit SSO-objekt. Udelad for anonyme brugere. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Side Reaktion Svarestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' når brugeren allerede har tilføjet denne reaktion. 'react-id-too-long' (HTTP 422) når id'et er over 36 tegn. Ellers inkluderet ved fejl. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Inkluderet ved fejl. **/
    reason?: string
}
[inline-code-end]

---