## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| urlId | string | Yes |  |
| sso | string | No |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример putReopenThread'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t urlId = utility::conversions::to_string_t("my-tenant-123/thread-456");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc123");
auto reopenTask = api->putReopenThread(urlId, sso)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) -> std::shared_ptr<APIEmptyResponse> {
        try {
            return t.get();
        } catch (...) {
            return std::make_shared<APIEmptyResponse>();
        }
    });
[inline-code-end]

---