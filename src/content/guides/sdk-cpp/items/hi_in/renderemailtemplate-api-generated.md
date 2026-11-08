## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | हाँ |  |
| locale | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/RenderEmailTemplateResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'renderEmailTemplate उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = RenderEmailTemplateBody();
body.templateId = U("welcome-email");
body.recipientEmail = U("user@example.com");
boost::optional<utility::string_t> locale = U("en-US");

api->renderEmailTemplate(U("my-tenant-123"), body, locale)
    .then([](std::shared_ptr<RenderEmailTemplateResponse> resp) {
        std::cout << "Email template rendered successfully\n";
    });
[inline-code-end]