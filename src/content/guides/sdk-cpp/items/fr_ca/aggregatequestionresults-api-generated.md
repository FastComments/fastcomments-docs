## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| questionId | string | Non |  |
| questionIds | vector<string | Non |  |
| urlId | string | Non |  |
| timeBucket | AggregateTimeBucket | Non |  |
| startDate | datetime | Non |  |
| forceRecalculate | bool | Non |  |

## Réponse

Renvoie : [`AggregateQuestionResults_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResults_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de aggregateQuestionResults'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> questionId(U("q-42"));
boost::optional<std::vector<utility::string_t>> questionIds(std::vector<utility::string_t>{U("q-42"), U("q-43")});
boost::optional<utility::string_t> urlId(U("page-789"));
boost::optional<AggregateTimeBucket> timeBucket;
boost::optional<utility::datetime> startDate(utility::datetime::from_string(U("2025-01-01T00:00:00Z")));
boost::optional<bool> forceRecalculate(true);

api->aggregateQuestionResults(tenantId, questionId, questionIds, urlId, timeBucket, startDate, forceRecalculate)
.then([](std::shared_ptr<AggregateQuestionResults_200_response> resp){
    if (resp) {
        auto resultCopy = std::make_shared<AggregateQuestionResults_200_response>(*resp);
    }
});
[inline-code-end]