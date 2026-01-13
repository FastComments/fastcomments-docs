## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateQuestionResultBody | UpdateQuestionResultBody | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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