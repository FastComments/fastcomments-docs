## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | double | לא |  |

## תגובה

מחזיר: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 20;
auto defaultResp = std::make_shared<GetTenantUsersResponse>();
api->getTenantUsers(tenantId, skip)
.then([defaultResp](std::shared_ptr<GetTenantUsersResponse> resp){
    auto result = resp ? resp : defaultResp;
    std::cout << (resp ? "Tenant users retrieved successfully\n" : "Using default response\n");
}).wait();
[inline-code-end]

---