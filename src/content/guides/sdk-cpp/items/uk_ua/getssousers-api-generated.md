## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| skip | int32_t | Ні |  |

## Відповідь

Повертає: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUsersResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getSSOUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = 25;
api->getSSOUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetSSOUsersResponse>> t) {
    try {
        auto response = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---