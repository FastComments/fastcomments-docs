## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Yes |  |

## Odgovor

Vraća: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateEmailTemplateResponse.h)

## Primjer

[inline-code-attrs-start title = 'createEmailTemplate Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<CreateEmailTemplateBody>();
body->subject = utility::string_t("Welcome Email");
body->htmlContent = utility::string_t("<h1>Welcome!</h1><p>Thanks for joining.</p>");
body->fromEmail = utility::string_t("no-reply@myapp.com");
body->replyTo = boost::optional<utility::string_t>(utility::string_t("support@myapp.com"));
body->cc = boost::none;
api->createEmailTemplate(utility::string_t("my-tenant-123"), *body)
    .then([](std::shared_ptr<CreateEmailTemplateResponse> resp) {
        std::cout << "Created template ID: " << resp->id << std::endl;
    });
[inline-code-end]