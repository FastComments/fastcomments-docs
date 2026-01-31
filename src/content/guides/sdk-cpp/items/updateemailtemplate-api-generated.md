## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateEmailTemplate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
auto bodyPtr = std::make_shared<UpdateEmailTemplateBody>();
bodyPtr->subject = U("Welcome to Acme Corp");
bodyPtr->htmlBody = U("<p>Hi {{name}}, welcome to Acme!</p>");
bodyPtr->replyTo = boost::optional<utility::string_t>(U("support@acme.com"));
bodyPtr->enabled = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, *bodyPtr)
.then([](std::shared_ptr<FlagCommentPublic_200_response> res){
    if (res) std::cout << "Email template updated successfully\n";
    else std::cout << "Failed to update template\n";
});
[inline-code-end]
