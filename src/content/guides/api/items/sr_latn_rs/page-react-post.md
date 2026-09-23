[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Dodaje reakciju na stranicu kao trenutni korisnik. Korisnik može dodati svaki ID reakcije samo jednom: ponovnim dodavanjem se uspešno vraća kod `already-reacted` i ne menja brojanje. Korisnik može dodati više različitih reakcija na istu stranicu.

ID‑ove reakcija biraš ti i mogu imati najviše 36 znakova. Stranica se kreira ako još ne postoji. Prosledi `title` da postaviš ili ažuriraš naslov stranice.

[inline-code-attrs-start title = 'Primer cURL zahteva za reakciju na stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za reakciju na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** ID reakcije, najviše 36 znakova. **/
    id: string
    /** Postavlja naslov stranice. **/
    title?: string
    /** URI‑enkodovan JSON vašeg SSO objekta. Omitujte za anonimne korisnike. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za reakciju na stranicu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' kada je korisnik već dodao ovu reakciju. 'react-id-too-long' (HTTP 422) kada je ID duži od 36 znakova. U suprotnom je uključeno u slučaju greške. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]