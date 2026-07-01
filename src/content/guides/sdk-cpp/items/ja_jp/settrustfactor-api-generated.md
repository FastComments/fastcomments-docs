---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const SetTrustFactorOptions& | Yes |  |

## レスポンス

戻り値: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetUserTrustFactorResponse.h)

## 例

[inline-code-attrs-start title = 'setTrustFactor の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
SetTrustFactorOptions opts;
opts.userId = utility::conversions::to_string_t("user@example.com");
opts.trustFactor = 8;
opts.reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("Spam check passed"));
api->setTrustFactor(utility::conversions::to_string_t("my-tenant-123"), opts)
    .then([](pplx::task<std::shared_ptr<SetUserTrustFactorResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---