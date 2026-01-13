[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ова рута омогућава уклањање `TenantUser` по id-у.

Брисање корисникових коментара је могуће посредством query параметра `deleteComments`. Имајте у виду да ако је ово тачно:

1. Сви корисникови коментари ће бити обрисани одмах.
2. Сви __child__ (сада сирачки) коментари ће бити обрисани или анонимизовани на основу конфигурације странице повезане са сваким коментаром. На пример ако је режим брисања нити "anonymize", онда ће одговори остати, а корисникови коментари ће бити анонимизовани. Ово важи само када је `commentDeleteMode` `Remove` (подразумевана вредност).
3. Вредност `creditsCost` постаје `2`.

### Анонимизовани коментари

Можете задржати корисникове коментаре али их једноставно анонимизовати подешавањем `commentDeleteMode=1`.

Ако су корисникови коментари анонимизовани, следеће вредности ће бити постављене на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` се постављају на `true`.

При приказу, видгет за коментаре ће користити `DELETED_USER_PLACEHOLDER` (подразумевано: "[deleted]") за име корисника и `DELETED_CONTENT_PLACEHOLDER` за коментар. Ово се може прилагодити преко корисничког интерфејса за прилагођавање видгета.

### Примјери

[inline-code-attrs-start title = 'Примјер cURL захтјева за уклањање TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за уклањање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // подразумевано
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Можете ово поставити на true да бисте такође обрисали корисникове коментаре. Ово ће удвостручити потрошњу кредита. **/
    deleteComments?: 'true' | 'false'
    /** Можете ово поставити по жељи да одредите како поступати са корисниковим коментарима. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]