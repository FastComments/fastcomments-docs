## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplate_200_response.h)

## Example

[inline-code-attrs-start title = 'getEmailTemplate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantOpt = utility::string_t(U("my-tenant-123"));
utility::string_t templateId = U("welcome-email-template");
api->getEmailTemplate(tenantOpt.value(), templateId)
.then([](std::shared_ptr<GetEmailTemplate_200_response> resp) {
    auto result = resp ? resp : std::make_shared<GetEmailTemplate_200_response>();
    if (result) {
        std::wcout << U("Received email template\n");
    } else {
        std::wcout << U("No template returned\n");
    }
});
[inline-code-end]
