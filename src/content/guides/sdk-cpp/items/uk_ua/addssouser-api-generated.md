## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createAPISSOUserData | CreateAPISSOUserData | Так |  |

## Відповідь

Повертає: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад addSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto createData = std::make_shared<CreateAPISSOUserData>();
createData->email = utility::string_t(U("alice@example.com"));
createData->externalUserId = utility::string_t(U("okta|987654321"));
createData->displayName = boost::optional<utility::string_t>(U("Alice Johnson"));
createData->roles = std::vector<utility::string_t>{ U("moderator"), U("editor") };
api->addSSOUser(tenantId, createData)
.then([](pplx::task<std::shared_ptr<AddSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            (void)resp;
        }
    } catch (...) {
    }
});
[inline-code-end]

---