## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## 응답

반환값: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'replaceTenantPackage 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---