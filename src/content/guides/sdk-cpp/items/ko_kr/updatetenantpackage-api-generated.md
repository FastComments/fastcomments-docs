## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'updateTenantPackage 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // success handling
        } catch (const std::exception& ex) {
            // error handling
        }
    });
[inline-code-end]

---