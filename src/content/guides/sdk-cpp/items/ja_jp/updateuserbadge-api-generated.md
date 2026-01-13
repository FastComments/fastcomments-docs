## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateUserBadgeParams | UpdateUserBadgeParams | はい |  |

## レスポンス

戻り値: [`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## 例

[inline-code-attrs-start title = 'updateUserBadge の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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