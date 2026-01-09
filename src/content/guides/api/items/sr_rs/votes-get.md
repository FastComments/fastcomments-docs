[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Гласови морају бити преузети по `urlId`.

### Врсте гласова

Постоје три врсте гласова:

- Аутентификовани гласови, који се примењују на одговарајући коментар. Можете их креирати помоћу овог API-ја.
- Аутентификовани гласови, који су у стању **на чекању** за верификацију, и стога још нису примењени на коментар. Ови се креирају када корисник користи FastComments.com *login to vote* механизам.
- Анонимни гласови, који се примењују на одговарајући коментар. Ови се креирају заједно са анонимним коментарисањем.

Они се у API-ју враћају у одвојеним списковима ради смањења забуне.

[inline-code-attrs-start title = 'Пример cURL захтева за гласове'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за гласове'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Овлашћени, верификовани гласови, примењени на одговарајуће коментаре. **/
    appliedAuthorizedVotes: Vote[]
    /** Анонимни гласови, примењени на одговарајуће коментаре. **/
    appliedAnonymousVotes: Vote[]
    /** Гласови на чекању верификације, још нису примењени на одговарајуће коментаре. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Напомене о анонимним гласовима

Имајте на уму да ће анонимни гласови креирани преко овог API-ја појавити у списку `appliedAuthorizedVotes`. Сматрају се овлашћеним јер су креирани преко API-ја помоћу API кључа.

Структура `appliedAnonymousVotes` је за гласове креиране без е-поште, API кључа итд.

---