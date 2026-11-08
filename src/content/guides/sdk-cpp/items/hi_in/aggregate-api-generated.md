दस्तावेज़ों को समूहित करके (यदि groupBy दिया गया है) एकत्रित करता है और कई ऑपरेशन लागू करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| aggregationRequest | AggregationRequest | हाँ |  |
| options | const AggregateOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/AggregateResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'aggregate उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

AggregationRequest aggregationRequest;
aggregationRequest.setMetric(U("commentCount"));
aggregationRequest.setStartDate(U("2023-01-01T00:00:00Z"));
aggregationRequest.setEndDate(U("2023-12-31T23:59:59Z"));
aggregationRequest.setFilters({ U("status:approved") });

AggregateOptions options;
options.limit = boost::optional<int>(100);
options.includeMetadata = boost::optional<bool>(true);

api->aggregate(tenantId, aggregationRequest, options)
    .then([](pplx::task<std::shared_ptr<AggregateResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया प्रोसेस करें
        } catch (const std::exception& e) {
            // त्रुटि संभालें
        }
    });
[inline-code-end]