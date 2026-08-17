Past commenters on the page who are NOT currently online. Sorted by displayName.  
Используйте это после исчерпания /users/online, чтобы отобразить раздел «Участники».  
Курсорная пагинация по commenterName: сервер проходит частичный {tenantId, urlId, commenterName} индекс от afterName вперёд через $gt, без затрат $skip.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | const GetOfflineUsersOptions& | Да |  |

## Response

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Example

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---