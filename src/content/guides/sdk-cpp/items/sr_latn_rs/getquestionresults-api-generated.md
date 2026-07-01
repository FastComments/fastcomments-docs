## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | const GetQuestionResultsOptions& | Da |  |

## Odgovor

Vraća: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetQuestionResultsOptions options;
options.questionId = boost::optional<utility::string_t>(U("question-456"));
options.includeDeleted = boost::optional<bool>(false);
api->getQuestionResults(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
        auto response = t.get();
        // obradi odgovor
    });
[inline-code-end]