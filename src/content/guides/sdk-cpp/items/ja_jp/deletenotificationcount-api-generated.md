## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteNotificationCount の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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