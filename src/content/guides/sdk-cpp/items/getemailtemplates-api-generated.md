## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Response

Returns: [`GetEmailTemplates_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplates_200_response.h)

## Example

[inline-code-attrs-start title = 'getEmailTemplates Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getEmailTemplates(tenantId, skip).then([=](std::shared_ptr<GetEmailTemplates_200_response> resp){
    auto result = resp ? resp : std::make_shared<GetEmailTemplates_200_response>();
    (void)result;
}).wait();
[inline-code-end]
