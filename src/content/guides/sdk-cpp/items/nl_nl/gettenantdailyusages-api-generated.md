## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| yearNumber | double | Nee |  |
| monthNumber | double | Nee |  |
| dayNumber | double | Nee |  |
| skip | double | Nee |  |

## Response

Retourneert: [`GetTenantDailyUsages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsages_200_response.h)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantDailyUsages Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> year = 2025;
boost::optional<double> month = 1;
auto placeholder = std::make_shared<GetTenantDailyUsages_200_response>();
api->getTenantDailyUsages(tenantId, year, month, boost::optional<double>(), boost::optional<double>())
.then([](pplx::task<std::shared_ptr<GetTenantDailyUsages_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Received tenant daily usages\n";
        else std::cout << "No usage data\n";
    } catch (const std::exception &e) {
        std::cout << "Request error: " << e.what() << '\n';
    }
});
[inline-code-end]

---