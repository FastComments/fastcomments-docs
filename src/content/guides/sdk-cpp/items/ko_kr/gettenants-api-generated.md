## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| options | const GetTenantsOptions& | 예 |  |

## 응답

반환: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantsResponse.h)

## 예제

[inline-code-attrs-start title = 'getTenants 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetTenantsOptions options;
options.includeDeleted = boost::make_optional(false);
options.searchTerm = boost::make_optional(utility::string_t(U("enterprise")));

api->getTenants(utility::string_t(U("my-tenant-123")), options)
    .then([](std::shared_ptr<GetTenantsResponse> response) {
    })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(...){} });
[inline-code-end]