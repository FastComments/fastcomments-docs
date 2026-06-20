## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("admin@company.com");
boost::optional<utility::string_t> includeMetadata = boost::optional<utility::string_t>(U("true"));
auto fallback = std::make_shared<GetTenantResponse>();
api->getTenant(tenantId, id).then([fallback](pplx::task<std::shared_ptr<GetTenantResponse>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? resp : fallback;
        std::cout << "Tenant retrieved successfully" << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "getTenant error: " << e.what() << std::endl;
    }
});
[inline-code-end]

---