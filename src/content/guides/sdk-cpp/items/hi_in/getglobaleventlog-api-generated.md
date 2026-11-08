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

रिटर्न करता है: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEventLogResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getGlobalEventLog उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getGlobalEventLog(
    U("my-tenant-123"),
    U("article-456"),
    U("user@example.com"),
    1622505600,
    boost::optional<int64_t>(1625097600)
).then([](std::shared_ptr<GetEventLogResponse> resp) {
    (void)resp;
});
[inline-code-end]