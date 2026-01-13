[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Этот маршрут удаляет `TenantUser` по id.

Удаление комментариев пользователя возможно через параметр запроса `deleteComments`. Обратите внимание, что если он установлен в true:

1. Все комментарии пользователя будут удалены в реальном времени.
2. Все __child__ (теперь сиротские) комментарии будут удалены или анонимизированы в соответствии с конфигурацией страницы, связанной с каждым комментарием. Например, если режим удаления ветки установлен в "anonymize", то ответы останутся, а комментарии пользователя будут анонимизированы. Это применяется только когда `commentDeleteMode` равен `Remove` (значение по умолчанию).
3. `creditsCost` становится `2`.

### Anonymized Comments

Вы можете сохранить комментарии пользователя, просто анонимизировав их, установив `commentDeleteMode=1`.

Если комментарии пользователя анонимизированы, то следующие поля устанавливаются в null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

Поля `isDeleted` и `isDeletedUser` устанавливаются в `true`.

При отрисовке виджет комментариев будет использовать `DELETED_USER_PLACEHOLDER` (по умолчанию: "[deleted]") для имени пользователя и `DELETED_CONTENT_PLACEHOLDER` для содержимого комментария. Это можно настроить через интерфейс настройки виджета.

### Примеры

[inline-code-attrs-start title = 'Пример cURL запроса удаления TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса удаления TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // по умолчанию
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Вы можете установить это в true, чтобы также удалить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    deleteComments?: 'true' | 'false'
    /** Вы можете установить это по своему усмотрению, чтобы определить, как обрабатывать комментарии пользователя. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на удаление TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]

---