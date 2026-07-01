## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| options | const GetSearchUsersOptions& | Так |  |

## Response

Повертає: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationUserSearchResponse.h)

## Example

[inline-code-attrs-start title = 'Приклад getSearchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetSearchUsersOptions opts;
opts.query = utility::string_t(U("john.doe@example.com"));
opts.page = boost::optional<int>(1);
opts.pageSize = boost::optional<int>(20);

api->getSearchUsers(tenantId, opts)
    .then([](std::shared_ptr<ModerationUserSearchResponse> resp) {
        for (const auto& user : resp->users) {
            std::wcout << user.id << L" - " << user.email << std::endl;
        }
    });
[inline-code-end]