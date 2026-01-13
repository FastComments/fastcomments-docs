## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | 是 |  |

## 回應

回傳: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 範例

[inline-code-attrs-start title = 'updateEmailTemplate 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-01");
UpdateEmailTemplateBody updateBody;
boost::optional<utility::string_t> optionalFrom = boost::optional<utility::string_t>(U("no-reply@myapp.com"));
updateBody.from = optionalFrom;
updateBody.subject = U("Welcome to MyApp");
updateBody.html = U("<p>Hi {{displayName}}, welcome to MyApp!</p>");
auto bodyPtr = std::make_shared<UpdateEmailTemplateBody>(updateBody);
api->updateEmailTemplate(tenantId, templateId, *bodyPtr)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "Email template updated successfully\n";
        } else {
            std::cout << "Unexpected empty response\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---