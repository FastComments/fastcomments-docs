[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут позволяет удалить одного SSO-пользователя по его id.

Обратите внимание, что повторная загрузка виджета комментариев с payload для этого пользователя просто вновь создаст пользователя.

Удаление комментариев пользователя возможно через параметр запроса `deleteComments`. Обратите внимание, что если это правда:

1. Все комментарии пользователя будут удалены в реальном времени.
2. Все __дочерние__ (теперь сиротские) комментарии будут удалены или анонимизированы в зависимости от конфигурации страницы, связанной с каждым комментарием. Например, если thread deletion mode равен "anonymize", то ответы останутся, а комментарии пользователя будут анонимизированы. Это применяется только когда `commentDeleteMode` установлен в `Remove` (значение по умолчанию).
3. `creditsCost` становится `2`.

### Анонимизированные комментарии

Вы можете сохранить комментарии пользователя, но просто анонимизировать их, установив `commentDeleteMode=1`.

Если комментарии пользователя анонимизированы, то следующие значения устанавливаются в null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` и `isDeletedUser` устанавливаются в `true`.

При рендеринге виджет комментариев будет использовать `DELETED_USER_PLACEHOLDER` (по умолчанию: "[deleted]") для имени пользователя и `DELETED_CONTENT_PLACEHOLDER` для комментария. Эти значения можно настроить через UI настройки виджета (Widget Customization UI).

### Примеры

[inline-code-attrs-start title = 'Пример cURL-запроса удаления SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса на удаление SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // по умолчанию
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Вы можете установить это в true, чтобы также удалить комментарии пользователя. Это удвоит стоимость в кредитах. **/
    deleteComments?: 'true' | 'false'
    /** Вы можете установить это по своему усмотрению, чтобы определить, как обрабатывать комментарии пользователя. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при удалении SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Включается при ошибке. **/
    reason?: string
    user?: SSOUser; // Мы возвращаем удаленного пользователя при успешном выполнении.
}
[inline-code-end]