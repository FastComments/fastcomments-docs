## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Ответ

Возвращает: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getTenantUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10;

api->getTenantUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantUsersResponse>> t){
    try {
        auto resp = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]