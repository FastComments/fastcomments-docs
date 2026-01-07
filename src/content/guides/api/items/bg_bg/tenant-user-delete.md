[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Този маршрут предоставя премахване на `TenantUser` по id.

Изтриването на коментарите на потребителя е възможно чрез параметъра на заявката `deleteComments`. Обърнете внимание, че ако това е true:

1. Всички коментари на потребителя ще бъдат изтрити на живо.
2. Всички __дъщерни__ (сега сираци) коментари ще бъдат изтрити или анонимизирани въз основа на конфигурацията на асоциираната страница на всеки коментар. Например, ако режимът на изтриване на нишка е "anonymize", тогава отговорите ще останат, а коментарите на потребителя ще бъдат анонимизирани. Това се прилага само когато `commentDeleteMode` е `Remove` (стойността по подразбиране).
3. `creditsCost` става `2`.

### Анонимизирани коментари

Можете да запазите коментарите на потребителя, но просто да ги анонимизирате, като зададете `commentDeleteMode=1`.

Ако коментарите на потребителя са анонимизирани, следните стойности се задават на null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` се задават на `true`.

При рендериране уиджетът за коментари ще използва `DELETED_USER_PLACEHOLDER` (по подразбиране: "[deleted]") за името на потребителя и `DELETED_CONTENT_PLACEHOLDER` за коментара. Те могат да бъдат персонализирани чрез UI за персонализиране на уиджета.

### Примери

[inline-code-attrs-start title = 'Пример за премахване на TenantUser с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за премахване на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за премахване на TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
