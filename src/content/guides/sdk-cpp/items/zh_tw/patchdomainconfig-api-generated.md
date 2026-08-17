## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## 回應

返回：[`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto domainToUpdate = utility::conversions::to_string_t("example.com");
PatchDomainConfigParams patchDomainConfigParams;
patchDomainConfigParams.enableComments = boost::optional<bool>(true);
patchDomainConfigParams.moderationLevel = boost::optional<int>(2);
patchDomainConfigParams.customHeader = boost::optional<utility::string_t>(utility::conversions::to_string_t("X-Custom-Header"));

api->patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        // 成功處理
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* 錯誤處理 */ }
    });
[inline-code-end]