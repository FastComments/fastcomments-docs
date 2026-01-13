[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава уклањање појединачног SSO корисника по његовом id-у.

Имајте на уму да поновно учитавање видгета за коментаре са подацима за овог корисника једноставно поново креира корисника без прекида.

Брисање корисникових коментара је могуће путем query параметра `deleteComments`. Имајте на уму да ако је ово постављено на true:

1. Сви корисникови коментари ће бити обрисани одмах.
2. Сви __child__ (сада сирочни) коментари ће бити обрисани или анонимизовани у зависности од конфигурације странице повезане са сваким коментаром. На примјер, ако је режим брисања теме "anonymize", онда одговори ће остати, а корисникови коментари ће бити анонимизовани. Ово важи само када је `commentDeleteMode` постављен на `Remove` (подразумјевана вриједност).
3. `creditsCost` постаје `2`.

### Анонимизовани коментари

Можете задржати корисникове коментаре али их једноставно анонимизовати постављањем `commentDeleteMode=1`.

Ако су корисникови коментари анонимизовани онда се сљедеће вриједности постављају на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` су постављени на `true`.

При приказу, видгет за коментаре ће користити `DELETED_USER_PLACEHOLDER` (подразумјевано: "[deleted]") за корисниково име и `DELETED_CONTENT_PLACEHOLDER` за коментар. Ово се може прилагодити преко корисничког интерфејса за прилагођавање видгета.

### Примјери

[inline-code-attrs-start title = 'cURL примјер за уклањање SSO корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за уклањање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // подразумјевано
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Можете ово поставити на true да бисте такође обрисали корисникове коментаре. Ово ће удвостручити трошак кредита. **/
    deleteComments?: 'true' | 'false'
    /** Можете ово поставити по потреби да одредите како поступити са корисниковим коментарима. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Укључено у случају неуспјеха. **/
    reason?: string
    user?: SSOUser; // Враћамо уклоњеног корисника у случају успјеха.
}
[inline-code-end]