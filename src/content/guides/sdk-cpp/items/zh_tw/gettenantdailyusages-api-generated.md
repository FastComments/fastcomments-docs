## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| yearNumber | double | 否 |  |
| monthNumber | double | 否 |  |
| dayNumber | double | 否 |  |
| skip | double | 否 |  |

## 回應

回傳：[`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsagesResponse.h)

## 範例

[inline-code-attrs-start title = 'getTenantDailyUsages 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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