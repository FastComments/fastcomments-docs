[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Этот маршрут обеспечивает удаление `TenantUser` по идентификатору.

Удаление комментариев пользователя возможно через параметр запроса `deleteComments`. Обратите внимание, что если он установлен в true:

1. Все комментарии пользователя будут удалены в реальном времени.
2. Все __дочерние__ (теперь осиротевшие) комментарии будут удалены или анонимизированы в зависимости от конфигурации страницы, связанной с каждым комментарием. Например, если режим удаления ветки — "anonymize", то ответы останутся, а комментарии пользователя будут анонимизированы. Это применяется только когда `commentDeleteMode` равен `Remove` (значение по умолчанию).
3. `creditsCost` становится `2`.

### Анонимизированные комментарии

Вы можете сохранить комментарии пользователя, просто анонимизировав их, установив `commentDeleteMode=1`.

Если комментарии пользователя анонимизированы, следующие значения устанавливаются в null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` устанавливаются в `true`.

При отображении виджет комментариев будет использовать `DELETED_USER_PLACEHOLDER` (по умолчанию: "[deleted]") для имени пользователя и `DELETED_CONTENT_PLACEHOLDER` для комментария. Их можно настроить через интерфейс настройки виджета.

### Примеры

[inline-code-attrs-start title = 'Пример удаления TenantUser с помощью cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Вы можете установить true, чтобы также удалить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    deleteComments?: 'true' | 'false'
    /** Вы можете установить это по своему усмотрению, чтобы определить, как обрабатывать комментарии пользователя. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа удаления TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]