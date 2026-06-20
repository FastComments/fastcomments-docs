## 매개변수

| Name | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 아니오 |  |
| userId | string | 아니오 |  |
| startDate | string | 아니오 |  |
| questionId | string | 아니오 |  |
| questionIds | string | 아니오 |  |
| skip | double | 아니오 |  |

## 응답

반환: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetQuestionResultsResponse.h)

## 예제

[inline-code-attrs-start title = 'getQuestionResults 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> urlId(U("page-456"));
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> startDate(U("2023-01-01T00:00:00Z"));
boost::optional<utility::string_t> questionId(U("q-789"));
boost::optional<utility::string_t> questionIds(U("q-789,q-790"));
boost::optional<double> skip(10.0);
api->getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip)
.then([](pplx::task<std::shared_ptr<GetQuestionResultsResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetQuestionResultsResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetQuestionResultsResponse>();
    }
});
[inline-code-end]

---