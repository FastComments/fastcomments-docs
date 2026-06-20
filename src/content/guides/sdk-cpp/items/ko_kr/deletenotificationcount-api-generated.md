## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteNotificationCount 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t countId = U("nc-00123");
boost::optional<utility::string_t> requestedBy = U("admin@company.com");

api->deleteNotificationCount(tenantId, countId)
.then([requestedBy](pplx::task<std::shared_ptr<APIEmptyResponse>> task) -> std::shared_ptr<APIEmptyResponse> {
    try {
        auto resp = task.get();
        (void)requestedBy;
        return resp ? resp : std::make_shared<APIEmptyResponse>();
    } catch (const std::exception&) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---