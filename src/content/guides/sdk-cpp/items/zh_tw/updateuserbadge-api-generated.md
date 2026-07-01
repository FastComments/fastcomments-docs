## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateUserBadgeParams | UpdateUserBadgeParams | 是 |  |

## 回應

回傳: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptySuccessResponse.h)

## 範例

[inline-code-attrs-start title = 'updateUserBadge 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
UpdateUserBadgeParams params;
params.badgeId = utility::conversions::to_string_t("vip-badge");
params.expiration = boost::optional<utility::datetime>(utility::datetime::from_string(U("2024-12-31T23:59:59Z")));
api->updateUserBadge(tenantId, userId, params)
    .then([](std::shared_ptr<APIEmptySuccessResponse> resp){
        std::cout << "Badge updated successfully\n";
    })
    .catch([](std::exception const& e){
        std::cerr << "Error updating badge: " << e.what() << "\n";
    });
[inline-code-end]