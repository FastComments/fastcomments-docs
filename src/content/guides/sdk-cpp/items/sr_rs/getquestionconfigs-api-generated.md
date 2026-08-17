## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| skip | double | Не |  |

## Одговор

Враћа: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## Пример

[inline-code-attrs-start title = 'getQuestionConfigs Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getQuestionConfigs(tenantId, skip).then([](std::shared_ptr<GetQuestionConfigsResponse> resp){
    auto config = std::make_shared<GetQuestionConfigsResponse>(*resp);
});
[inline-code-end]

---