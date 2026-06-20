## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| yearNumber | double | Όχι |  |
| monthNumber | double | Όχι |  |
| dayNumber | double | Όχι |  |
| skip | double | Όχι |  |

## Response

Επιστρέφει: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsagesResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantDailyUsages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> yearNumber = 2026;
boost::optional<double> monthNumber = 6;
boost::optional<double> dayNumber; 
boost::optional<double> skip = 0;
api->getTenantDailyUsages(tenantId, yearNumber, monthNumber, dayNumber, skip)
.then([=](pplx::task<std::shared_ptr<GetTenantDailyUsagesResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetTenantDailyUsagesResponse>();
        return resp;
    } catch(...) {
        return std::make_shared<GetTenantDailyUsagesResponse>();
    }
});
[inline-code-end]

---