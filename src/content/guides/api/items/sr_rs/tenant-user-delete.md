[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ова рута омогућава уклањање `TenantUser` по id.

Брисање корисникових коментара је могуће помоћу query параметра `deleteComments`. Имајте на уму да ако је ово тачно:

1. Сви корисникови коментари биће одмах обрисани.
2. Сви __child__ (сада осирочени) коментари биће обрисани или анонимизовани у зависности од конфигурације странице повезане са сваким коментаром. На пример, ако је режим брисања нити "anonymize", онда ће одговори остати, а корисникови коментари ће бити анонимизовани. Ово се односи само када је `commentDeleteMode` постављен на `Remove` (подразумевана вредност).
3. `creditsCost` постаје `2`.

### Анонимизовани коментари

Можете задржати корисникове коментаре, али их једноставно анонимизовати постављањем `commentDeleteMode=1`.

Ако су кориснички коментари анонимизовани, следеће вредности се постављају на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` се постављају на `true`.

При приказивању, виџет коментара ће користити `DELETED_USER_PLACEHOLDER` (подразумевано: "[deleted]") за име корисника и `DELETED_CONTENT_PLACEHOLDER` за коментар. Ово се може прилагодити преко UI за прилагођавање виџета.

### Примери

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за уклањање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // подразумевано
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Можете поставити ово на true да бисте такође избрисали корисничке коментаре. Ово ће удвостручити трошак у кредитима. **/
    deleteComments?: 'true' | 'false'
    /** Ово можете подесити по жељи да одредите како поступати са корисничким коментарима. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---