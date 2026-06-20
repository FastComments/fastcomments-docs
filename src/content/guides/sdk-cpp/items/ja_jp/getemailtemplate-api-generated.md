---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateResponse.h)

## 例

[inline-code-attrs-start title = 'getEmailTemplate の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));

api->getEmailTemplate(tenantId, templateId).then([locale](pplx::task<std::shared_ptr<GetEmailTemplateResponse>> t) {
    try {
        auto resp = t.get();
        auto localCopy = std::make_shared<GetEmailTemplateResponse>(*resp);
        std::cout << "Email template fetched: " << (resp ? "success" : "null") << std::endl;
        if (locale) std::cout << "Locale: " << locale->c_str() << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Failed to get template: " << e.what() << std::endl;
    }
});
[inline-code-end]

---