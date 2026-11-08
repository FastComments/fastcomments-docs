## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetQuestionResultsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionResults उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetQuestionResultsOptions options;
options.questionId = boost::optional<utility::string_t>(U("question-456"));
options.includeDeleted = boost::optional<bool>(false);
api->getQuestionResults(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
        auto response = t.get();
        // प्रतिक्रिया को संभालें
    });
[inline-code-end]