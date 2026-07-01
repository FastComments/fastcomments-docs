## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## 応答

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateTenantPackage の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<UpdateTenantPackageBody>();
body->packageId = utility::conversions::to_string_t("premium-plan");
body->expirationDate = utility::conversions::to_string_t("2025-12-31");
body->notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Upgraded package"));

api->updateTenantPackage(utility::conversions::to_string_t("my-tenant-123"),
                         utility::conversions::to_string_t("pkg-456"),
                         body)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
        try {
            auto response = task.get();
            // 成功時の処理
        } catch (const std::exception& ex) {
            // エラー時の処理
        }
    });
[inline-code-end]