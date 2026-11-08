## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getTenantUsers उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10;

api->getTenantUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantUsersResponse>> t){
    try {
        auto resp = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]