## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Tak |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateQuestionResult'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t questionId = utility::conversions::to_string_t("question-456");
auto body = std::make_shared<UpdateQuestionResultBody>();
body->answeredBy = utility::conversions::to_string_t("user@example.com");
body->correct = true;
body->score = boost::optional<int>(92);
body->notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Clarified response during review"));
api->updateQuestionResult(tenantId, questionId, *body)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try {
            auto resp = t.get();
            (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---