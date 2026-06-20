## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 예제

[inline-code-attrs-start title = 'deleteV1PageReact 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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