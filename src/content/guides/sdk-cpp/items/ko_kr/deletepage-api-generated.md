## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'deletePage 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-456");
boost::optional<utility::string_t> correlationId = boost::optional<utility::string_t>(U("corr-20251122"));
auto task = api->deletePage(tenantId, pageId)
    .then([correlationId](pplx::task<std::shared_ptr<DeletePageAPIResponse>> prev) -> std::shared_ptr<DeletePageAPIResponse> {
        try {
            auto resp = prev.get();
            if (resp) return resp;
            return std::make_shared<DeletePageAPIResponse>();
        } catch (const std::exception&) {
            return std::make_shared<DeletePageAPIResponse>();
        }
    });
[inline-code-end]

---