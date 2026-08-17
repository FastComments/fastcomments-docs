## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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