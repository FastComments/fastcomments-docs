## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 예 |  |
| locale | string | 아니요 |  |

## 응답

반환: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplateResponse.h)

## 예제

[inline-code-attrs-start title = 'renderEmailTemplate 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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