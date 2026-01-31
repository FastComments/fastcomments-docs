## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| skip | double | No |  |

## Response

Returns: [`GetEmailTemplateRenderErrors_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrors_200_response.h)

## Example

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("email-template-789");
boost::optional<double> skip = boost::optional<double>(10);
auto defaultResp = std::make_shared<GetEmailTemplateRenderErrors_200_response>();
api->getEmailTemplateRenderErrors(tenantId, templateId, skip)
    .then([defaultResp](std::shared_ptr<GetEmailTemplateRenderErrors_200_response> resp) {
        auto result = resp ? resp : defaultResp;
        if (result) std::cout << "Render errors retrieved for template\n";
        else std::cout << "No data\n";
    });
[inline-code-end]
