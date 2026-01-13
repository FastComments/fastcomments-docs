## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 是 |  |

## 回應

回傳: [`CreateEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateEmailTemplate_200_response.h)

## 範例

[inline-code-attrs-start title = 'createEmailTemplate 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto body = std::make_shared<CreateEmailTemplateBody>();
body->name = utility::string_t(U("Welcome Email"));
body->subject = utility::string_t(U("Welcome to Our Service"));
body->fromEmail = utility::string_t(U("no-reply@mycompany.com"));
body->fromName = utility::string_t(U("MyCompany Support"));
body->html = utility::string_t(U("<p>Hello {{user.name}}, welcome to MyCompany.</p>"));
body->description = boost::optional<utility::string_t>(utility::string_t(U("Onboarding welcome template")));
api->createEmailTemplate(tenantId, *body)
.then([](pplx::task<std::shared_ptr<CreateEmailTemplate_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            bool created = true;
            (void)created;
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---