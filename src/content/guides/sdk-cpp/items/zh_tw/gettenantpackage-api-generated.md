---
## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳：[`GetTenantPackage_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackage_200_response.h)

## 範例

[inline-code-attrs-start title = 'getTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
boost::optional<utility::string_t> requestTrace = U("trace-20260112-01");
api->getTenantPackage(tenantId, packageId)
.then([requestTrace](std::shared_ptr<GetTenantPackage_200_response> resp){
    auto result = resp ? resp : std::make_shared<GetTenantPackage_200_response>();
    if (requestTrace) { auto trace = *requestTrace; (void)trace; }
    return result;
})
.then([](std::shared_ptr<GetTenantPackage_200_response> finalResp){
    (void)finalResp;
});
[inline-code-end]

---