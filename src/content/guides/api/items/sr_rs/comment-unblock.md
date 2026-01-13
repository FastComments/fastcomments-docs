[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Овај API endpoint омогућава де-блокирање корисника који је написао одређени коментар. Подржава де-блокирање на основу коментара које су написали FastComments.com Users, SSO Users и Tenant Users.

Подржава параметар у телу захтева `commentIdsToCheck` да провери да ли неки други потенцијално видљиви коментари на клијенту треба да буду блокирани/де-блокирани након извршене радње.

Напомене:

- Овај позив мора увек бити извршен у контексту корисника. Корисник може бити FastComments.com User, SSO User или Tenant User.
- `userId` у захтеву је корисник који *врши де-блокирање*. На пример: `User A` wants to Un-Block `User B`. Проследите `userId=User A` и ID коментара који је написао `User B`.
- Потпуно анонимни коментари (без ID-а корисника, без е-поште) не могу бити блокирани и биће враћена грешка.

[inline-code-attrs-start title = 'Primer cURL zahteva za deblokiranje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteva za deblokiranje anonimnog komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za deblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za deblokiranje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Ako je commentIdsToCheck definisan, unosi u ovoj mapi са true су и даље блокирани. Ако су false, можда ћете пожелети да поново прикажете коментаре кориснику тако да не мора да освежава страницу. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]