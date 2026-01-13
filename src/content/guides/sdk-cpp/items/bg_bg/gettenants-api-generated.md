## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| meta | string | Не |  |
| skip | double | Не |  |

## Отговор

Връща: [`GetTenants_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenants_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример на getTenants'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> meta = boost::optional<utility::string_t>(U("admin@example.com"));
boost::optional<double> skip = boost::optional<double>(10.0);
auto task = api->getTenants(tenantId, meta, skip)
    .then([](std::shared_ptr<GetTenants_200_response> resp){
        if (resp) {
            auto copy = std::make_shared<GetTenants_200_response>(*resp);
        }
    });
[inline-code-end]

---