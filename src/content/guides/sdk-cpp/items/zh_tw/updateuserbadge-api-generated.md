## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 是 |  |

## 回應

回傳：[`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## 範例

[inline-code-attrs-start title = 'updateUserBadge 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
UpdateUserBadgeParams updateParams;
updateParams.badgeId = U("badge-top-100");
updateParams.label = U("Top Contributor");
updateParams.active = boost::optional<bool>(true);
updateParams.expiresAt = boost::optional<utility::string_t>(U("2026-12-31T23:59:59Z"));
api->updateUserBadge(tenantId, id, updateParams)
.then([](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto result = std::make_shared<UpdateUserBadge_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---