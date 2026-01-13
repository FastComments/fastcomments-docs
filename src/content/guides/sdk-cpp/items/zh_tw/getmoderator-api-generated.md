## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳：[`GetModerator_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModerator_200_response.h)

## 範例

[inline-code-attrs-start title = 'getModerator 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
boost::optional<utility::string_t> requestLocale = U("en-US");
auto fallbackModerator = std::make_shared<GetModerator_200_response>();

api->getModerator(tenantId, moderatorId)
    .then([fallbackModerator, requestLocale](std::shared_ptr<GetModerator_200_response> resp) -> pplx::task<std::shared_ptr<GetModerator_200_response>> {
        auto moderator = resp ? resp : fallbackModerator;
        if (requestLocale) { auto loc = *requestLocale; (void)loc; }
        return pplx::task_from_result(moderator);
    })
    .then([](std::shared_ptr<GetModerator_200_response> finalResp) -> void {
        (void)finalResp;
    });
[inline-code-end]

---