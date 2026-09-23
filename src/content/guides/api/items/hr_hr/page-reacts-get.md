[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj za svaku reakciju na stranici i koje je reakcije trenutni korisnik dodao.

[inline-code-attrs-start title = 'Primjer cURL za reakcije na stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za reakcije na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI kodirani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za reakcije na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: string
    /** Uključeno pri neuspjehu. **/
    reason?: string
    /** Broj po ID-ju reakcije, npr. {"heart": 12, "laugh": 3}. Nije postavljeno kada stranica nema reakcija. **/
    counts?: Record<string, number>
    /** ID-ji reakcija koje je korisnik poslao zahtjev dodao. Nije postavljeno kada nijednu nije dodao. **/
    reactedIds?: string[]
}
[inline-code-end]