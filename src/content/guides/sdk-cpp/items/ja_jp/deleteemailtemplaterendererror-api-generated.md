## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| errorId | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-template-456");
utility::string_t errorId = U("render-error-789");
auto placeholder = std::make_shared<FlagCommentPublic_200_response>();
boost::optional<utility::string_t> operatorNote = boost::none;
operatorNote = U("auto-resolved");

api->deleteEmailTemplateRenderError(tenantId, templateId, errorId)
    .then([operatorNote, templateId](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> previous) {
        try {
            auto resp = previous.get();
            if (resp) {
                std::cout << "Successfully deleted render error for template: " << utility::conversions::to_utf8string(templateId) << std::endl;
            }
            if (operatorNote) {
                std::cout << "Note: " << utility::conversions::to_utf8string(operatorNote.value()) << std::endl;
            }
        } catch (const std::exception &e) {
            std::cerr << "API error: " << e.what() << std::endl;
        }
    });
[inline-code-end]

---