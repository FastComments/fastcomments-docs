## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | vector<string | No |  |
| urlId | string | No |  |
| timeBucket | AggregateTimeBucket | No |  |
| startDate | datetime | No |  |
| forceRecalculate | bool | No |  |

## Response

Returns: [`AggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResults_200_response.h)

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("question-456"));
boost::optional<std::vector<utility::string_t>> questionIds = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{U("question-456"), U("question-789")});
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("https://example.com/articles/123"));
boost::optional<AggregateTimeBucket> timeBucket = boost::optional<AggregateTimeBucket>(AggregateTimeBucket::DAILY);
boost::optional<utility::datetime> startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2024-01-01T00:00:00Z")));
boost::optional<bool> forceRecalculate = boost::optional<bool>(true);
api->aggregateQuestionResults(tenantId, questionId, questionIds, urlId, timeBucket, startDate, forceRecalculate)
.then([](std::shared_ptr<AggregateQuestionResults_200_response> resp){
    auto result = resp ? resp : std::make_shared<AggregateQuestionResults_200_response>();
    (void)result;
});
[inline-code-end]
