## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 回應

回傳: [`GetTenantPackages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackages_200_response.h)

## 範例

[inline-code-attrs-start title = 'getTenantPackages 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
auto task = api->getTenantPackages(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantPackages_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            (void)resp;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<GetTenantPackages_200_response>();
        (void)fallback;
    }
});
[inline-code-end]

---