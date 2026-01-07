[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Позволява извличане на гласове, оставени от потребител за даден `urlId`. Приема `userId`, който може да бъде всеки потребител на FastComments.com или `SSO User`.

Това е полезно, ако искате да покажете дали потребител е гласувал за коментар. Когато извличате коментари, просто извикайте този API едновременно за потребителя със
същия `urlId`.

Ако използвате анонимно гласуване, ще искате да подадете `anonUserId` вместо това.

[inline-code-attrs-start title = 'Пример за Votes за потребител с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Пример за Votes за анонимен потребител с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Обърнете внимание, че анонимните гласове ще се появят в списъка `appliedAuthorizedVotes`. Те се считат за оторизирани, тъй като са създадени чрез API с API ключ.

[inline-code-attrs-start title = 'Структура на заявката за Votes за потребител'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за Votes за потребител'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
