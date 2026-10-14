[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Vraća broj za svaku reakciju na stranici, i po želji i korisnike koji su reagovali, kao jedan unos po reakciji sa njegovim ID‑om reakcije, najnoviji prvi. Ovo je server‑side verzija [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` kontroliše koji se korisnici vraćaju:

- `public` izostavlja korisnike koji su učinili svoju aktivnost profila privatnom.
- `include-private` uključuje ih. Koristite ovo samo kada lista nije prikazana javnosti.

Anonimni korisnici se nikada ne uključuju, a pretražuje se do 100 reakcija. Stranice koje ne postoje ne vraćaju `counts` i vraćaju praznu listu `users`.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Izostavite da biste dobili samo brojeve. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Broj po ID‑u reakcije, na primer {"heart": 12, "laugh": 3}. Nije postavljeno kada stranica nema reakcije. **/
    counts?: Record<string, number>
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

#### Korisni savet

Zapamtite da URI‑enkodujete vrednosti poput `urlId`.

---