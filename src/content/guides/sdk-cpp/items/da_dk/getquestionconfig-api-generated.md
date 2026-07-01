## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getQuestionConfig Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto questionId = utility::conversions::to_string_t("question-456");

api->getQuestionConfig(tenantId, questionId)
    .then([](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> task) {
        try {
            auto response = task.get();
            // Brug svaret efter behov
        } catch (const std::exception&) {
            // Håndter fejl
        }
    });
[inline-code-end]