## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Sí |  |

## Respuesta

Devuelve: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateEmailTemplateResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateEmailTemplateBody>();
bodyPtr->name = U("Welcome Email");
bodyPtr->subject = U("Welcome to MyApp");
bodyPtr->fromEmail = U("no-reply@myapp.com");
bodyPtr->htmlTemplate = U("<h1>Welcome, \{{user.name}}!</h1>");
bodyPtr->plainText = boost::optional<utility::string_t>(U("Welcome, \{{user.name}}!"));
bodyPtr->replyTo = boost::optional<utility::string_t>(U("support@myapp.com"));
api->createEmailTemplate(tenantId, *bodyPtr)
.then([](pplx::task<std::shared_ptr<CreateEmailTemplateResponse>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Created template ID: " << resp->id << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Create failed: " << e.what() << std::endl;
    }
});
[inline-code-end]