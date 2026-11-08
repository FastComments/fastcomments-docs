## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createQuestionResultBody | CreateQuestionResultBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateQuestionResultResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'createQuestionResult उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateQuestionResultBody body;
body.questionId = U("question-456");
body.result = U("approved");
body.comment = boost::optional<utility::string_t>(U("Looks good"));
api->createQuestionResult(tenantId, body)
    .then([=](pplx::task<std::shared_ptr<CreateQuestionResultResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]