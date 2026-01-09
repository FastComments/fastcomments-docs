[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава уклањање једног SSO корисника по његовом id-у.

Имајте у виду да поновно учитавање видгета коментара са payload-ом за овог корисника једноставно ће поново креирати корисника без прекида.

Брисање корисникових коментара је могуће помоћу `deleteComments` query параметра. Имајте у виду да ако је ово true:

1. Сви корисникови коментари биће обрисани уживо.
2. Сви __child__ (сада сирочад) коментари биће обрисани или анонимизовани у зависности од конфигурације странице повезане са сваким коментаром. На пример, ако је режим брисања нити "anonymize", онда одговори остају, а корисникови коментари ће бити анонимизовани. Ово се примењује само када је `commentDeleteMode` `Remove` (подразумевана вредност).
3. `creditsCost` постаје `2`.

### Анонимизовани коментари

Можете задржати корисникове коментаре али их једноставно анонимизовати постављањем `commentDeleteMode=1`.

Ако су корисникови коментари анонимизовани онда следеће вредности се постављају на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` се постављају на `true`.

При рендеровању, видгет коментара ће за име корисника користити `DELETED_USER_PLACEHOLDER` (подразумевано: "[deleted]") и `DELETED_CONTENT_PLACEHOLDER` за сам коментар. Ово се може прилагодити преко корисничког интерфејса за прилагођавање видгета.

### Примери

[inline-code-attrs-start title = 'Пример cURL захтева за уклањање SSO корисника'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за уклањање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // подразумевано
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање SSO корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Укључено при неуспеху. **/
    reason?: string
    user?: SSOUser; // На успех враћамо уклоњеног корисника.
}
[inline-code-end]