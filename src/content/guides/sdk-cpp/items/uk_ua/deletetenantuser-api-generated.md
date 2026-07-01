## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| options | const DeleteTenantUserOptions& | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'deleteTenantUser Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DeleteTenantUserOptions options;
options.reason = boost::optional<utility::string_t>(U("User requested deletion"));

api->deleteTenantUser(U("my-tenant-123"), U("user@example.com"), options)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        (void)resp;
    });
[inline-code-end]