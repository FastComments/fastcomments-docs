## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const CombineCommentsWithQuestionResultsOptions& | Да |  |

## Одговор

Враћа: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CombineQuestionResultsWithCommentsResponse.h)

## Пример

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // Користите комбинирано по потреби
        }catch(const std::exception&){
        }
    });
[inline-code-end]