[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Гласовете трябва да бъдат извлечени по `urlId`.

### Типове гласове

Има три типа гласове:

- Удостоверени гласове, които се прилагат към съответния коментар. Можете да ги създадете чрез този API.
- Удостоверени гласове, които са **в очакване** на верификация и следователно все още не са приложени към коментара. Те се създават, когато потребител използва механизма *влизане за гласуване* на FastComments.com.
- Анонимни гласове, които се прилагат към съответния коментар. Те се създават заедно с анонимното коментиране.

Те се връщат в отделни списъци в API, за да се намали объркването.

[inline-code-attrs-start title = 'Пример за Votes с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Бележки за анонимни гласове

Обърнете внимание, че анонимните гласове, създадени чрез този API, ще се появят в списъка `appliedAuthorizedVotes`. Те се считат за оторизирани, тъй като са създадени чрез API с API ключ.

Структурата `appliedAnonymousVotes` е за гласове, създадени без имейл, API ключ и т.н.
