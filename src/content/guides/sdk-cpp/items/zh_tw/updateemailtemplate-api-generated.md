## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Yes |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'updateEmailTemplate 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email");
UpdateEmailTemplateBody body;
body.subject = U("Welcome to Our Platform");
body.content = U("<p>Hello \{{userName}}, welcome aboard!</p>");
body.isActive = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, body)
    .then([](std::shared_ptr<APIEmptyResponse> response) {
        // 成功處理
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception &) { /* 錯誤處理 */ }
    });
[inline-code-end]