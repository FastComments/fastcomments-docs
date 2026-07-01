## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Відповідь

Повертає: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## Приклад

[inline-code-attrs-start title = 'getSubscriptions Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenant = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> user = utility::conversions::to_string_t("user@example.com");

api->getSubscriptions(tenant, user).then(
    [](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t) {
        try {
            auto response = t.get();
            // обробити відповідь
        } catch (const std::exception& e) {
            // обробка помилки
        }
    }
);
[inline-code-end]

---