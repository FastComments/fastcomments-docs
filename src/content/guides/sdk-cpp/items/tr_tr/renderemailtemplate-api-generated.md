## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Evet |  |
| locale | string | Hayır |  |

## Yanıt

Döndürür: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplateResponse.h)

## Örnek

[inline-code-attrs-start title = 'renderEmailTemplate Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto body = std::make_shared<RenderEmailTemplateBody>();
body->templateId = U("welcome-email");
body->recipientEmail = U("user@example.com");
boost::optional<utility::string_t> locale = U("en-US");
api->renderEmailTemplate(tenantId, *body, locale)
    .then([](pplx::task<std::shared_ptr<RenderEmailTemplateResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                std::cout << "Rendered email template received for tenant\n";
            }
        } catch (const std::exception& e) {
            std::cerr << "Error: " << e.what() << '\n';
        }
    });
[inline-code-end]

---