## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| yearNumber | double | No |  |
| monthNumber | double | No |  |
| dayNumber | double | No |  |
| skip | double | No |  |

## Response

Returns: [`GetTenantDailyUsages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsages_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenantDailyUsages Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> yearNumber = 2025;
boost::optional<double> monthNumber = 1;
boost::optional<double> dayNumber = 15;
boost::optional<double> skip;
api->getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip)
.then([](pplx::task<std::shared_ptr<GetTenantDailyUsages_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetTenantDailyUsages_200_response>();
        std::cout << "Received tenant daily usages result\n";
    } catch (const std::exception &e) {
        std::cerr << "Request failed: " << e.what() << '\n';
    }
}).wait();
[inline-code-end]
