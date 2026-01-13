[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава уклањање појединачног SSO корисника по његовом id-у.

Имајте на уму да поновно учитавање видгета за коментаре са payload-ом за овог корисника једноставно ће поново креирати корисника непримјетно.

Брисање корисникових коментара је могуће преко query параметра `deleteComments`. Имајте на уму да ако је то `true`:

1. Сви корисникови коментари биће обрисани одмах.
2. Сви __child__ (сада осиротјели) коментари биће обрисани или анонимизовани у складу са конфигурацијом странице повезане са сваким коментаром. На примјер, ако је режим брисања теме "anonymize", онда ће одговори остати, а корисникови коментари ће бити анонимизовани. Ово се односи само када је `commentDeleteMode` `Remove` (подразумевана вриједност).
3. Вриједност `creditsCost` постаје `2`.

### Анонимизовани коментари

Можете сачувати корисникове коментаре али једноставно их анонимизовати постављањем `commentDeleteMode=1`.

Ако су корисникови коментари анонимизовани, тада се сљедеће вриједности постављају на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` се постављају на `true`.

При приказивању, видгет за коментаре ће користити `DELETED_USER_PLACEHOLDER` (подразумевано: "[deleted]") за име корисника и `DELETED_CONTENT_PLACEHOLDER` за садржај коментара. Ово се може прилагодити преко корисничког интерфејса за прилагођавање видгета.

### Примјери

[inline-code-attrs-start title = 'Пример cURL захтјева за уклањање SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за уклањање SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // подразумевано
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Можете поставити ово на true да бисте такође обрисали корисникове коментаре. Ово ће удвостручити трошак кредита. **/
    deleteComments?: 'true' | 'false'
    /** Можете ово подесити по потреби да одредите како поступати са корисниковим коментарима. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за уклањање SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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