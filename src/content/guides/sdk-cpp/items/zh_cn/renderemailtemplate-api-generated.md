## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 是 |  |
| locale | string | 否 |  |

## 响应

返回：[`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/RenderEmailTemplateResponse.h)

## 示例

[inline-code-attrs-start title = 'renderEmailTemplate 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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