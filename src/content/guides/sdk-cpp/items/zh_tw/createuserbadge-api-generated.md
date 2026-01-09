## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createUserBadgeParams | CreateUserBadgeParams | 是 |  |

## 回應

回傳: [`CreateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateUserBadge_200_response.h)

## 範例

[inline-code-attrs-start title = 'createUserBadge 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateUserBadgeParams params;
params.userId = U("user@example.com");
params.badgeId = U("trusted-contributor");
params.note = boost::optional<utility::string_t>(U("Awarded for outstanding moderation"));
api->createUserBadge(tenantId, params)
.then([](std::shared_ptr<CreateUserBadge_200_response> resp){
    if (resp) {
        auto copied = std::make_shared<CreateUserBadge_200_response>(*resp);
    }
})
.wait();
[inline-code-end]

---