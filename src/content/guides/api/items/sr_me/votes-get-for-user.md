[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Ово омогућава преузимање гласова које је корисник оставио за дати `urlId`. Захтева `userId` који може бити било који FastComments.com или `SSO User`.

Ово је корисно ако желите да прикажете да ли је корисник гласао за коментар. При преузимању коментара, једноставно позовите овај API у исто време за тог корисника са истим `urlId`.

Ако користите анонимно гласање, уместо тога пошаљите `anonUserId`.

[inline-code-attrs-start title = 'Пример cURL захтева за гласове корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за анонимног корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Имајте у виду да ће анонимни гласови бити приказани у листи `appliedAuthorizedVotes`. Сматрају се овлашћеним јер су креирани преко API-ја уз помоћ API кључа.

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
    /** Гласови који чекају верификацију, још нису примењени на одговарајуће коментаре. **/
    pendingVotes: Vote[]
}
[inline-code-end]