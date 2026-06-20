## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteTenantPackage 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
boost::optional<utility::string_t> ifMatch = U("W/\"etag-789\"");

api->deleteTenantPackage(tenantId, packageId)
.then([=](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto finalResp = resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (const std::exception& e) {
        auto errorResp = std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]