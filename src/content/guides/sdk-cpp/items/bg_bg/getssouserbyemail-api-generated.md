## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| email | string | Да |  |

## Отговор

Връща: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByEmailAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getSSOUserByEmail'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t email = U("user@example.com");
boost::optional<utility::string_t> includeInactive = boost::optional<utility::string_t>(U("false"));
api->getSSOUserByEmail(tenantId, email).then([includeInactive](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> t) {
    try {
        auto resp = t.get();
        return resp ? resp : std::make_shared<GetSSOUserByEmailAPIResponse>();
    } catch (...) {
        return std::make_shared<GetSSOUserByEmailAPIResponse>();
    }
}).then([](std::shared_ptr<GetSSOUserByEmailAPIResponse> finalResp) {
    (void)finalResp;
});
[inline-code-end]

---