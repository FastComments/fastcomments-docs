## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| urlId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'putReopenThread 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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