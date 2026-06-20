## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModeratorResponse.h)

## 範例

[inline-code-attrs-start title = 'getModerator 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenant = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> moderatorId = utility::string_t(U("moderator-456"));
api->getModerator(tenant.value(), moderatorId.value())
    .then([](pplx::task<std::shared_ptr<GetModeratorResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) return std::make_shared<GetModeratorResponse>(*resp);
            return std::shared_ptr<GetModeratorResponse>();
        } catch (...) {
            return std::shared_ptr<GetModeratorResponse>();
        }
    })
    .then([](std::shared_ptr<GetModeratorResponse> result) {
        if (result) {
            /* 使用結果 */
        }
    });
[inline-code-end]

---