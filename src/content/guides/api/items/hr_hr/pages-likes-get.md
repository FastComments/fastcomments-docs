[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Vraća broj lajkova na stranici, i po želji korisnike koji su je lajkali, najnoviji prvi. Ovo je poslužiteljska verzija [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` kontrolira koji se korisnici vraćaju:

- `public` izostavlja korisnike koji su učinili svoju aktivnost profila privatnom.
- `include-private` uključuje ih. Koristite ovo samo kada popis nije prikazan javnosti.

Anonimni korisnici se nikada ne uključuju, a pretražuje se najviše 100 lajkova. Stranice koje ne postoje vraćaju `likeCount` od `0` i prazan popis `users`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za lajkove stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Izostavite da biste dobili samo broj lajkova. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    likeCount?: number
    /** Najnoviji prvi. Postavlja se samo kada je proslijeđen includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Korisni savjet

Zapamtite da URI‑enkodirate vrijednosti poput `urlId`.

---