## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Відповідь

Повертає: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutDomainConfigResponse.h)

## Приклад

[inline-code-attrs-start title = 'putDomainConfig Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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