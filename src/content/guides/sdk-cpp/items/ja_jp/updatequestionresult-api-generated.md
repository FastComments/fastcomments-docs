## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateQuestionResultBody | UpdateQuestionResultBody | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'updateQuestionResult の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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