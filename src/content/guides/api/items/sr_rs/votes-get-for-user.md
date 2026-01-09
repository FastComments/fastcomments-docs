[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Омогућава преузимање гласова које је оставио корисник за одређени `urlId`. Прима `userId` који може бити било који FastComments.com корисник или `SSO User`.

Ово је корисно ако желите да прикажете да ли је корисник гласао за коментар. При преузимању коментара, једноставно позовите овај API истовремено за тог корисника са истим `urlId`.

Ако користите анонимно гласање, онда треба да проследите `anonUserId` уместо тога.

[inline-code-attrs-start title = 'Пример cURL захтева за гласове корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за гласове анонимног корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Имајте у виду да ће анонимни гласови бити приказани у листи `appliedAuthorizedVotes`. Они се сматрају овлашћеним јер су креирани путем API-ја са API key-ом.

[inline-code-attrs-start title = 'Структура захтева за гласове корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за гласове корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Овлашћени, верификовани гласови, примењени на одговарајуће коментаре. **/
    appliedAuthorizedVotes: Vote[]
    /** Гласови који чекају верификацију, још увек нису примењени на своје одговарајуће коментаре. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---