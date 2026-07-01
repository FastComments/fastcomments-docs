## Parameter

| Name   | Type                                          | Required | Beschreibung |
|--------|-----------------------------------------------|----------|--------------|
| tenantId | string                                        | Ja       |              |
| options  | const CombineCommentsWithQuestionResultsOptions& | Ja       |              |

## Antwort

Rückgabe: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CombineQuestionResultsWithCommentsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
CombineCommentsWithQuestionResultsOptions options;
options.questionId = utility::string_t("question-789");
options.maxComments = boost::optional<int>(50);
api->combineCommentsWithQuestionResults(tenantId, options).then(
    [](pplx::task<std::shared_ptr<CombineQuestionResultsWithCommentsResponse>> task){
        try{
            auto respPtr = task.get();
            auto combined = std::make_shared<CombineQuestionResultsWithCommentsResponse>(*respPtr);
            // Verwenden Sie combined nach Bedarf
        }catch(const std::exception&){
        }
    });
[inline-code-end]