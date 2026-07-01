## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

戻り値: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeResponse.h)

## 例

[inline-code-attrs-start title = 'getUserBadge の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
api->getUserBadge(tenantId, userId).then([](pplx::task<std::shared_ptr<APIGetUserBadgeResponse>> t){
    try{
        auto resp = t.get();
        boost::optional<std::string> badgeUrl = resp->badge_url ? boost::optional<std::string>(*resp->badge_url) : boost::none;
    }catch(const std::exception&){
    }
});
[inline-code-end]