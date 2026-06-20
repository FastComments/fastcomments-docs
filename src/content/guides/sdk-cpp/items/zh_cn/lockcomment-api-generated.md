## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'lockComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-987654321");
utility::string_t broadcastId = utility::conversions::to_string_t("live-456");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
auto lockTask = api->lockComment(tenantId, commentId, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) -> std::shared_ptr<APIEmptyResponse> {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<APIEmptyResponse>();
            return resp;
        } catch (...) {
            return std::make_shared<APIEmptyResponse>();
        }
    });
[inline-code-end]

---