## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | כן |  |

## Response

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Example

[inline-code-attrs-start title = 'דוגמת updateEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email");
UpdateEmailTemplateBody body;
body.subject = U("Welcome to Our Platform");
body.content = U("<p>Hello \{{userName}}, welcome aboard!</p>");
body.isActive = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, body)
    .then([](std::shared_ptr<APIEmptyResponse> response) {
        // טיפול בהצלחה
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception &) { /* טיפול בשגיאה */ }
    });
[inline-code-end]

---