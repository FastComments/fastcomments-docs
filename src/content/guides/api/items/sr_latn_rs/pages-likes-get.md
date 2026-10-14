[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Vraća broj lajkova na stranici, i po želji i korisnike koji su je lajkovali, najnoviji prvi. Ovo je server‑side verzija [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` kontroliše koji se korisnici vraćaju:

- `public` izostavlja korisnike koji su učinili svoju aktivnost profila privatnom.
- `include-private` uključuje ih. Koristite ovo samo kada lista nije prikazana javnosti.

Anonimni korisnici se nikada ne uključuju, a pretražuje se do 100 lajkova. Stranice koje ne postoje vraćaju `likeCount` od `0` i prazan `users` spisak.

[inline-code-attrs-start title = 'Primer cURL zahteva za Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Izostavite da biste dobili samo broj lajkova. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno u slučaju greške. **/
    reason?: string
    likeCount?: number
    /** Najnoviji prvi. Postavlja se samo kada je prosleđen includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Korisni savet

Zapamtite da URI‑enkodujete vrednosti poput `urlId`.

---