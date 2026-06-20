## 参数

| Name | Type | 必填 | Description |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| questionId | string | 否 |  |
| questionIds | vector<string | 否 |  |
| urlId | string | 否 |  |
| timeBucket | AggregateTimeBucket | 否 |  |
| startDate | datetime | 否 |  |
| forceRecalculate | bool | 否 |  |

## 响应

返回：[`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResultsResponse.h)

## 示例

[inline-code-attrs-start title = 'aggregateQuestionResults 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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