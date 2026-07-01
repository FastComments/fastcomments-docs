## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetQuestionResultsOptions& | Yes |  |

## Response

Враћа: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## Example

[inline-code-attrs-start title = 'Primer getQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetQuestionResultsOptions options;
options.questionId = boost::optional<utility::string_t>(U("question-456"));
options.includeDeleted = boost::optional<bool>(false);
api->getQuestionResults(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
        auto response = t.get();
        // обработи одговор
    });
[inline-code-end]