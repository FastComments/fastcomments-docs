[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj za svaku reakciju na stranici i koje je reakcije trenutni korisnik dodao.

Proslijedite `includeUserInfo=public` da biste također dobili korisnike koji su reagovali, kao jedan unos po reakciji sa njegovim ID-om reakcije, najnoviji prvi. Korisnici s privatnom aktivnošću profila i anonimni korisnici su izostavljeni, a pretražuje se do 100 reakcija. Ovde se prihvata samo `public`; `include-private` vraća `422` i dostupan je putem [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) uz API ključ. Na stranicama koje zahtevaju SSO, `includeUserInfo` vraća `403` za korisnike koji ne mogu da vide stranicu.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI enkodovani JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
    /** Postavite na 'public' da biste uključili korisnike koji su reagovali. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Broj po ID-ju reakcije, na primer {"heart": 12, "laugh": 3}. Nije postavljeno kada stranica nema reakcije. **/
    counts?: Record<string, number>
    /** ID-ovi reakcija koje je korisnik koji je poslao zahtev dodao. Nije postavljeno ako nijednu nije dodao. **/
    reactedIds?: string[]
    /** Jedan unos po reakciji, najnoviji prvi. Postavlja se samo kada je prosleđen includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** ID reakcije, na primer "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

---