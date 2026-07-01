## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döner: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantResponse.h)

## Örnek

[inline-code-attrs-start title = 'getTenant Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto id = utility::conversions::to_string_t("tenant-admin-456");
boost::optional<utility::string_t> includeDetails = utility::conversions::to_string_t("full");
api->getTenant(tenantId, id)
    .then([](std::shared_ptr<GetTenantResponse> resp) {
        if (resp) {
            std::wcout << resp->toString() << std::endl;
        }
    });
[inline-code-end]