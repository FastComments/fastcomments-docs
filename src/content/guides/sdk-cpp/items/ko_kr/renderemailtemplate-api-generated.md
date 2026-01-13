---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 예 |  |
| locale | string | 아니오 |  |

## 응답

반환: [`RenderEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplate_200_response.h)

## 예제

[inline-code-attrs-start title = 'renderEmailTemplate 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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