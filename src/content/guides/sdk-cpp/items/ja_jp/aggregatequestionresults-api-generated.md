## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const AggregateQuestionResultsOptions& | Yes |  |

## レスポンス

返り値: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateQuestionResultsResponse.h)

## 例

[inline-code-attrs-start title = 'aggregateQuestionResults の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
AggregateQuestionResultsOptions opts;
opts.questionId = utility::conversions::to_string_t("question-789");
opts.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z"), utility::datetime::ISO_8601));
opts.endDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z"), utility::datetime::ISO_8601));
api->aggregateQuestionResults(tenantId, opts)
    .then([](std::shared_ptr<AggregateQuestionResultsResponse> resp) {
        static_cast<void>(resp);
    });
[inline-code-end]