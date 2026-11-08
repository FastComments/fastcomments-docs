req
tenantId
urlId
userIdWS

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| userIdWS | string | हाँ |  |
| startTime | int64_t | हाँ |  |
| endTime | int64_t | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getEventLog उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto startTime = int64_t(1622505600);
boost::optional<int64_t> endTime = int64_t(1622592000);
api->getEventLog(U("my-tenant-123"), U("article-456"), U("user@example.com"), startTime, endTime)
    .then([](std::shared_ptr<GetEventLogResponse> response){
        auto copy = std::make_shared<GetEventLogResponse>(*response);
        (void)copy;
    });
[inline-code-end]