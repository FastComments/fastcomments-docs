## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## 回應

回傳: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## 範例

[inline-code-attrs-start title = 'putDomainConfig 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
UpdateDomainConfigParams updateParams;
updateParams.enableComments = true;
updateParams.moderationLevel = boost::optional<int>(2);
updateParams.customCss = boost::optional<utility::string_t>(U(".fc-comment{color:red;}"));
api->putDomainConfig(tenantId, domainToUpdate, updateParams)
    .then([](std::shared_ptr<PutDomainConfigResponse> resp) {
    });
[inline-code-end]