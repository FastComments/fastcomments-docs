## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createQuestionResultBody | CreateQuestionResultBody | כן |  |

## תגובה

מחזיר: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateQuestionResultResponse.h)

## דוגמה

[inline-code-attrs-start title = 'createQuestionResult דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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