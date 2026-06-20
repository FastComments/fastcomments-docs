## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 返回

返回： [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## 示例

[inline-code-attrs-start title = 'getTenantUsers 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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