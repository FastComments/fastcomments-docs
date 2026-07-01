## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto questionId = utility::string_t(U("question-789"));
boost::optional<utility::string_t> optionalParam = boost::none;

api->getQuestionResult(tenantId, questionId)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]