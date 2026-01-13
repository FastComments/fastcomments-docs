## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| errorId | string | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Przykład

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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