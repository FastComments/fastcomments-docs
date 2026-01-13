## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| yearNumber | double | Нет |  |
| monthNumber | double | Нет |  |
| dayNumber | double | Нет |  |
| skip | double | Нет |  |

## Ответ

Возвращает: [`GetTenantDailyUsages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsages_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getTenantDailyUsages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> year = 2025;
boost::optional<double> month = 1;
auto placeholder = std::make_shared<GetTenantDailyUsages_200_response>();
api->getTenantDailyUsages(tenantId, year, month, boost::optional<double>(), boost::optional<double>())
.then([](pplx::task<std::shared_ptr<GetTenantDailyUsages_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Received tenant daily usages\n";
        else std::cout << "No usage data\n";
    } catch (const std::exception &e) {
        std::cout << "Request error: " << e.what() << '\n';
    }
});
[inline-code-end]

---