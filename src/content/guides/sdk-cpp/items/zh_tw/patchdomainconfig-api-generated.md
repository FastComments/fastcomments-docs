## 參數

| 名稱 | 類型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 是 |  |
| patchDomainConfigParams | PatchDomainConfigParams | 是 |  |

## 回應

回傳：[`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchDomainConfigResponse.h)

## 範例

[inline-code-attrs-start title = 'patchDomainConfig 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
PatchDomainConfigParams patchParams;
patchParams.enableComments = boost::optional<bool>(true);
patchParams.moderatorEmail = boost::optional<utility::string_t>(U("moderator@example.com"));
auto task = api->patchDomainConfig(tenantId, domainToUpdate, patchParams)
    .then([](std::shared_ptr<PatchDomainConfigResponse> resp) {
        auto result = resp ? resp : std::make_shared<PatchDomainConfigResponse>();
        return result;
    });
task.wait();
[inline-code-end]

---