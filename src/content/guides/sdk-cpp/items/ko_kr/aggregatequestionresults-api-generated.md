## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | vector<string | 아니오 |  |
| urlId | string | 아니오 |  |
| timeBucket | AggregateTimeBucket | 아니오 |  |
| startDate | datetime | 아니오 |  |
| forceRecalculate | bool | 아니오 |  |

## 응답

반환: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResultsResponse.h)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("question-42"));
boost::optional<std::vector<utility::string_t>> questionIds = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{U("question-42"), U("question-84")});
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("https://www.example.com/articles/123"));
boost::optional<AggregateTimeBucket> timeBucket = boost::optional<AggregateTimeBucket>(AggregateTimeBucket::Daily);
boost::optional<utility::datetime> startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2024-06-01T00:00:00Z")));
boost::optional<bool> forceRecalculate = boost::optional<bool>(true);

api->aggregateQuestionResults(tenantId, questionId, questionIds, urlId, timeBucket, startDate, forceRecalculate)
.then([](pplx::task<std::shared_ptr<AggregateQuestionResultsResponse>> task){
    try {
        auto resp = task.get();
        auto safeResp = resp ? resp : std::make_shared<AggregateQuestionResultsResponse>();
        (void)safeResp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---