## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | double | 아니오 |  |

## 응답

반환: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## 예제

[inline-code-attrs-start title = 'getTenantUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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