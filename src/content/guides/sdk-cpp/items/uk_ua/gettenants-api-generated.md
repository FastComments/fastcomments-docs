## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| meta | string | Ні |  |
| skip | double | Ні |  |

## Відповідь

Повертає: [`GetTenants_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenants_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenants'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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