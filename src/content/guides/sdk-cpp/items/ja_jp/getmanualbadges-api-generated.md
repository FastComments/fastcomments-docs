## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## 例

[inline-code-attrs-start title = 'getManualBadges の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("https://auth.example.com/sso?tenant=my-tenant-123&user=user@example.com"));
api->getManualBadges(sso).then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<GetTenantManualBadgesResponse>(*resp);
        }
    } catch (const std::exception& ex) {
        (void)ex;
    }
});
[inline-code-end]

---