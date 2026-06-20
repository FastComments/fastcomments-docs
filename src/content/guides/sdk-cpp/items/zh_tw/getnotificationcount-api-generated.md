## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | bool | 否 |  |
| type | string | 否 |  |

## 回應

回傳：[`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("article-456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("mention");

auto task = api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
.then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) return std::make_shared<GetNotificationCountResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetNotificationCountResponse>();
    }
});
[inline-code-end]

---