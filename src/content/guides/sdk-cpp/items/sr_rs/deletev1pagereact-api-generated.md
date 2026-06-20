## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Одговор

Враћа: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Пример

[inline-code-attrs-start title = 'Пример deleteV1PageReact'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-9876");
boost::optional<utility::string_t> requestId = U("req-456");
auto task = api->deleteV1PageReact(tenantId, urlId)
    .then([requestId](pplx::task<std::shared_ptr<CreateV1PageReact>> t) {
        try {
            auto result = t.get();
            if (!result) result = std::make_shared<CreateV1PageReact>();
            return result;
        } catch (const std::exception&) {
            return std::make_shared<CreateV1PageReact>();
        }
    });
task.wait();
[inline-code-end]

---