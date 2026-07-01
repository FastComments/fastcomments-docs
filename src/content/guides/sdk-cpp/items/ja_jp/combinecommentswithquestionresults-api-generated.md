## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const CombineCommentsWithQuestionResultsOptions& | Yes |  |

## レスポンス

返却: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CombineQuestionResultsWithCommentsResponse.h)

## 例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // 必要に応じて combined を使用
        }catch(const std::exception&){
        }
    });
[inline-code-end]