## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Yes |  |
| locale | string | No |  |

## Response

Returns: [`RenderEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplate_200_response.h)

## Example

[inline-code-attrs-start title = 'renderEmailTemplate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
RenderEmailTemplateBody body;
body.templateId = utility::conversions::to_string_t("welcome-email");
body.recipientEmail = utility::conversions::to_string_t("user@example.com");
body.subject = utility::conversions::to_string_t("Welcome to Acme Inc");

boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(utility::conversions::to_string_t("en-US"));

auto defaultResp = std::make_shared<RenderEmailTemplate_200_response>();

api->renderEmailTemplate(utility::conversions::to_string_t("my-tenant-123"), body, locale)
.then([defaultResp](std::shared_ptr<RenderEmailTemplate_200_response> resp){
    auto result = resp ? resp : defaultResp;
    (void)result;
    return pplx::task_from_result(result);
});
[inline-code-end]
