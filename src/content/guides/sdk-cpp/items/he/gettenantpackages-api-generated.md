## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | double | לא |  |

## תגובה

מחזיר: [`GetTenantPackages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackages_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantPackages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
auto task = api->getTenantPackages(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantPackages_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            (void)resp;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<GetTenantPackages_200_response>();
        (void)fallback;
    }
});
[inline-code-end]