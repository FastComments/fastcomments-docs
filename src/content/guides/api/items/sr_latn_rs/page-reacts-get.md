[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj za svaku reakciju na stranici i koje reakcije je trenutni korisnik dodao.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI enkodovani JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: string
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Broj po ID-ju reakcije, na primer {"heart": 12, "laugh": 3}. Nije postavljeno kada stranica nema reakcije. **/
    counts?: Record<string, number>
    /** ID-ovi reakcija koje je korisnik poslao zahteva dodao. Nije postavljeno kada nijednu nije dodao. **/
    reactedIds?: string[]
}
[inline-code-end]

---