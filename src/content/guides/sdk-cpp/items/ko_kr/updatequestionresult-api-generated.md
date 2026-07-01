## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateQuestionResultBody | UpdateQuestionResultBody | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예시

[inline-code-attrs-start title = 'updateQuestionResult 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto questionId = utility::string_t(U("question-456"));
UpdateQuestionResultBody body;
body.result = U("approved");
body.note = boost::optional<utility::string_t>(U("Reviewed by admin"));
api->updateQuestionResult(tenantId, questionId, body)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto respPtr = std::make_shared<APIEmptyResponse>(*t.get());
        } catch (const std::exception&) {
        }
    });
[inline-code-end]