---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| yearNumber | double | いいえ |  |
| monthNumber | double | いいえ |  |
| dayNumber | double | いいえ |  |
| skip | double | いいえ |  |

## レスポンス

戻り値: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantDailyUsagesResponse.h)

## 例

[inline-code-attrs-start title = 'getTenantDailyUsages の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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