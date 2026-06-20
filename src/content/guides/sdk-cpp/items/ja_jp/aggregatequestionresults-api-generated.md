## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| questionId | string | いいえ |  |
| questionIds | vector<string | いいえ |  |
| urlId | string | いいえ |  |
| timeBucket | AggregateTimeBucket | いいえ |  |
| startDate | datetime | いいえ |  |
| forceRecalculate | bool | いいえ |  |

## レスポンス

戻り値: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResultsResponse.h)

## 例

[inline-code-attrs-start title = 'aggregateQuestionResults の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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