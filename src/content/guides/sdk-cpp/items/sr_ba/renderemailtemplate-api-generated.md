## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Да |  |
| locale | string | Не |  |

## Одговор

Враћа: [`RenderEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplate_200_response.h)

## Примјер

[inline-code-attrs-start title = 'renderEmailTemplate Примјер'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
RenderEmailTemplateBody renderEmailTemplateBody;
renderEmailTemplateBody.setTemplateName(U("welcome_wagon"));
renderEmailTemplateBody.setRecipient(U("alice@example.com"));
web::json::value vars;
vars[U("firstName")] = web::json::value::string(U("Alice"));
renderEmailTemplateBody.setVariables(vars);
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
auto placeholder = std::make_shared<RenderEmailTemplate_200_response>();
api->renderEmailTemplate(tenantId, renderEmailTemplateBody, locale)
.then([](pplx::task<std::shared_ptr<RenderEmailTemplate_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Email template rendered successfully" << std::endl;
        } else {
            std::cerr << "No response returned" << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "Render failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---