## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Приклад

[inline-code-attrs-start title = 'Приклад updateQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("question-456");
UpdateQuestionResultBody updateQuestionResultBody;
auto moderatorEmail = std::make_shared<utility::string_t>(U("moderator@example.com"));
boost::optional<utility::string_t> resolutionNote = boost::optional<utility::string_t>(U("Marked duplicate of q-123"));
api->updateQuestionResult(tenantId, id, updateQuestionResultBody)
    .then([moderatorEmail, resolutionNote](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t)
    {
        try {
            auto resp = t.get();
            if (resp) {
                auto resultPtr = resp;
            }
        } catch (...) {}
    });
[inline-code-end]

---