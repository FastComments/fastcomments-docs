## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwoord

Geeft terug: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionResult Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t questionId = U("question-789");
boost::optional<utility::string_t> requestedBy = boost::optional<utility::string_t>(U("user@example.com"));

api->getQuestionResult(tenantId, questionId)
.then([requestedBy](pplx::task<std::shared_ptr<GetQuestionResultResponse>> task) -> std::shared_ptr<GetQuestionResultResponse> {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<GetQuestionResultResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetQuestionResultResponse>();
    }
});
[inline-code-end]

---