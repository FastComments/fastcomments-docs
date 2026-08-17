## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| userId | string | Не |  |

## Одговор

Враћа: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getSubscriptions'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenant = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> user = utility::conversions::to_string_t("user@example.com");

api->getSubscriptions(tenant, user).then(
    [](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t) {
        try {
            auto response = t.get();
            // обради одговор
        } catch (const std::exception& e) {
            // обради грешку
        }
    }
);
[inline-code-end]