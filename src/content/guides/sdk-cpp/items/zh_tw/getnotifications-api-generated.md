## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| urlId | string | 否 |  |
| fromCommentId | string | 否 |  |
| viewed | bool | 否 |  |
| type | string | 否 |  |
| skip | double | 否 |  |

## 回傳

回傳: [`GetNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotifications_200_response.h)

## 範例

[inline-code-attrs-start title = 'getNotifications 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("https://example.com/posts/42"));
boost::optional<utility::string_t> fromCommentId = boost::optional<utility::string_t>(U("cmt-98765"));
boost::optional<bool> viewed = boost::optional<bool>(true);
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(U("reply"));
boost::optional<double> skip = boost::optional<double>(0.0);

api->getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip)
.then([](pplx::task<std::shared_ptr<GetNotifications_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto processed = std::make_shared<GetNotifications_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---