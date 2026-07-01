## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetUserInternalProfileOptions& | Да |  |

## Ответ

Возвращает: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserInternalProfileResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getUserInternalProfile'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetUserInternalProfileOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
options.includeDetails = boost::optional<bool>(true);

api->getUserInternalProfile(tenantId, options)
    .then([](std::shared_ptr<GetUserInternalProfileResponse> response) {
        if (response) {
            auto name = response->displayName;
            auto id = response->userId;
        }
    });
[inline-code-end]