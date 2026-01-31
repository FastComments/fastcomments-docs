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
boost::optional<utility::string_t> questionId = boost::optional<utility::string_t>(U("q-456"));
std::vector<utility::string_t> qids = { U("q-456"), U("q-789") };
boost::optional<std::vector<utility::string_t>> questionIds = boost::optional<std::vector<utility::string_t>>(qids);
boost::optional<utility::string_t> urlId = boost::optional<utility::string_t>(U("page-42"));
AggregateTimeBucket bucket = AggregateTimeBucket::DAY;
boost::optional<AggregateTimeBucket> timeBucket = boost::optional<AggregateTimeBucket>(bucket);
boost::optional<utility::datetime> startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2025-01-01T00:00:00Z")));
boost::optional<bool> forceRecalculate = boost::optional<bool>(true);

api->aggregateQuestionResults(tenantId, questionId, questionIds, urlId, timeBucket, startDate, forceRecalculate)
.then([](pplx::task<std::shared_ptr<AggregateQuestionResults_200_response>> t){
    try {
        auto resp = t.get();
        auto resultCopy = std::make_shared<AggregateQuestionResults_200_response>(*resp);
    } catch (const std::exception&) {
    }
});
[inline-code-end]
