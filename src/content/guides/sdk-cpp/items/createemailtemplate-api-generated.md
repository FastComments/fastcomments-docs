## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Response

Returns: [`CreateEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateEmailTemplate_200_response.h)

## Example

[inline-code-attrs-start title = 'createEmailTemplate Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto body = std::make_shared<CreateEmailTemplateBody>();
body->name = utility::string_t(U("Welcome Email"));
body->subject = utility::string_t(U("Welcome to MyApp"));
body->html = utility::string_t(U("<p>Hello {{user_name}}, welcome to MyApp!</p>"));
boost::optional<utility::string_t> replyTo = boost::optional<utility::string_t>(U("support@example.com"));
body->replyTo = replyTo;
api->createEmailTemplate(tenantId, *body)
.then([](pplx::task<std::shared_ptr<CreateEmailTemplate_200_response>> task) {
  try {
    auto resp = task.get();
    if (resp) return resp;
  } catch (const std::exception&) {}
  return std::shared_ptr<CreateEmailTemplate_200_response>();
});
[inline-code-end]
