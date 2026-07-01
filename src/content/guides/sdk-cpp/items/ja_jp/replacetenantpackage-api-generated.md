## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| id | string | はい |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'replaceTenantPackage の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pkgId = utility::conversions::to_string_t("pkg-456");
ReplaceTenantPackageBody body;
body.planId = utility::conversions::to_string_t("enterprise");
body.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Upgrade request"));
api->replaceTenantPackage(tenantId, pkgId, body)
    .then([](std::shared_ptr<APIEmptyResponse>) { })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) { }
    });
[inline-code-end]