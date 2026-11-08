## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetQuestionConfigResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto questionId = utility::conversions::to_string_t("question-456");

api->getQuestionConfig(tenantId, questionId)
    .then([](pplx::task<std::shared_ptr<GetQuestionConfigResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया का आवश्यकतानुसार उपयोग करें
        } catch (const std::exception&) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]