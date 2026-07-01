## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'replaceTenantPackage Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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