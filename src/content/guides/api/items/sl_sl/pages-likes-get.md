[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

Vrne število všečkov na strani in po želji tudi uporabnike, ki so jo všečkali, najprej najnovejše. To je strežniška različica [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get).

`includeUserInfo` nadzoruje, kateri uporabniki so vrnjeni:

- `public` izpusti uporabnike, ki so svojo aktivnost profila nastavili kot zasebno.
- `include-private` jih vključi. Uporabite to le, ko seznam ni prikazan javnosti.

Anonimni uporabniki niso nikoli vključeni, in se pregleda do 100 všečkov. Strani, ki ne obstajajo, vrnejo `likeCount` vrednost `0` in prazen seznam `users`.

[inline-code-attrs-start title = 'Primer cURL zahteve za všečke strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za všečke strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Izpusti, da dobiš samo število všečkov. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za všečke strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Vključeno ob napaki. **/
    reason?: string
    likeCount?: number
    /** Najprej najnovejši. Nastavljeno le, ko je podan includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Koristen nasvet

Ne pozabite URI kodirati vrednosti, kot je `urlId`.