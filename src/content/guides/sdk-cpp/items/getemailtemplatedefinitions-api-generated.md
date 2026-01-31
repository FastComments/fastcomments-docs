## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitions_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitions_200_response.h)

## Example

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> preferredLocale = boost::optional<utility::string_t>(U("en-US"));
api->getEmailTemplateDefinitions(tenantId).then([=](std::shared_ptr<GetEmailTemplateDefinitions_200_response> resp) {
    auto result = resp ? std::make_shared<GetEmailTemplateDefinitions_200_response>(*resp)
                       : std::make_shared<GetEmailTemplateDefinitions_200_response>();
    return pplx::task_from_result(result);
}).then([](std::shared_ptr<GetEmailTemplateDefinitions_200_response> finalResp) {
    (void)finalResp;
});
[inline-code-end]
