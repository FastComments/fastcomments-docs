## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | double | 아니오 |  |

## 응답

반환: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackagesResponse.h)

## 예제

[inline-code-attrs-start title = 'getTenantPackages 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 20.0;
auto placeholder = std::make_shared<GetTenantPackagesResponse>();
api->getTenantPackages(tenantId, skip).then([placeholder](pplx::task<std::shared_ptr<GetTenantPackagesResponse>> t) {
    try {
        auto resp = t.get();
        std::cout << "Received packages: " << (resp ? "yes" : "no") << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error fetching packages: " << e.what() << std::endl;
    }
});
[inline-code-end]

---