[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Гласови се морају дохватити по `urlId`.

### Типови гласова

Постоје три врсте гласова:

- Аутентификовани гласови, који се примјењују на одговарајући коментар. Можете их креирати преко овог API-ја.
- Аутентификовани гласови, који чекају верификацију (**pending**), и стога још нису примјењени на коментар. Ови се креирају када корисник користи FastComments.com механизм *пријава за гласање*.
- Анонимни гласови, који се примјењују на одговарајући коментар. Они се креирају уз анонимно коментарисање.

Они се враћају у посебним списковима у API-ју да би се смањила конфузија.

[inline-code-attrs-start title = 'Примјер cURL захтјева'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за гласове'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за гласове'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Овлашћени, верификовани гласови, примјењени на одговарајуће коментаре. **/
    appliedAuthorizedVotes: Vote[]
    /** Анонимни гласови, примјењени на одговарајуће коментаре. **/
    appliedAnonymousVotes: Vote[]
    /** Гласови који чекају верификацију, још нису примјењени на одговарајуће коментаре. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Напомене о анонимним гласовима

Имајте у виду да ће анонимни гласови креирани преко овог API-ја појавити у листи `appliedAuthorizedVotes`. Они се сматрају овлашћеним јер су креирани преко API-ја уз API key.

Структура `appliedAnonymousVotes` је за гласове креиране без е-поште, API key-а итд.