[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава додавање једног овлашћеног `Vote`. Гласови могу бити `up` (+1) или `down` (-1).

[inline-code-attrs-start title = 'Пример cURL захтева за креирање гласа'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL захтева за креирање анонимног гласа'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за креирање гласа'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за креирање гласа'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Укључено у случају неуспеха. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Крeирање анонимних гласова

Анонимни гласови се могу креирати постављањем `anonUserId` у query параметрима уместо `userId`.

Овај id не мора да одговара објекту корисника нигде (отуда анониман). Он је једноставно идентификатор
за сесију, тако да можете поново преузети гласове у истој сесији, да бисте проверавали да ли је коментар
гласан.

Ако немате такву ствар као „анонимне сесије“ као што FastComments има — можете једноставно
поставити ово на случајни ID, као што је UUID (иако ценимо мање идентификаторе ради уштеде простора).

### Друге напомене

- Овај API поштује поставке на нивоу tenant-а. На пример, ако онемогућите гласање за одређену страницу, и покушате да креирате глас преко API-ја, то ће не успети са кодом грешке `voting-disabled`.
- Овај API је подразумевано live.
- Овај API ће ажурирати `votes` одговарајућег `Comment`.