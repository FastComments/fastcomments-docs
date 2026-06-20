---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| sso | string | Не |  |

## Отговор

Връща: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за getManualBadges'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("https://auth.example.com/sso?tenant=my-tenant-123&user=user@example.com"));
api->getManualBadges(sso).then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<GetTenantManualBadgesResponse>(*resp);
        }
    } catch (const std::exception& ex) {
        (void)ex;
    }
});
[inline-code-end]

---