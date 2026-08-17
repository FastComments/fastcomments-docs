## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример за updateTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // обрада успеха
        } catch (const std::exception& ex) {
            // обрада грешке
        }
    });
[inline-code-end]