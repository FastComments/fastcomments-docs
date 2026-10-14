[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Vrne število za vsako reakcijo na strani in po želji tudi uporabnike, ki so reagirali, kot en vnos na reakcijo z njenim ID-jem, najnovejše najprej. To je strežniška različica [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` nadzoruje, kateri uporabniki so vrnjeni:

- `public` izpusti uporabnike, ki so svojo aktivnost profila nastavili kot zasebno.
- `include-private` jih vključi. Uporabite to le, ko seznam ni prikazan javnosti.

Anonimni uporabniki niso nikoli vključeni, in do 100 reakcij se pregleda. Strani, ki ne obstajajo, ne vrnejo `counts` in vrnejo prazen seznam `users`.

[inline-code-attrs-start title = 'Primer cURL zahteve za reakcije strani'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za reakcije strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Izpusti, da dobiš samo števila. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za reakcije strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Število po ID-ju reakcije, na primer {"heart": 12, "laugh": 3}. Ni nastavljeno, ko stran nima reakcij. **/
    counts?: Record<string, number>
    /** En vnos na reakcijo, najnovejše najprej. Nastavljeno le, ko je podan includeUserInfo. **/
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

#### Koristni nasvet

Ne pozabite URI kodirati vrednosti, kot je `urlId`.