## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const CombineCommentsWithQuestionResultsOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CombineQuestionResultsWithCommentsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // combined का आवश्यकतानुसार उपयोग करें
        }catch(const std::exception&){
        }
    });
[inline-code-end]