## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlIdWS | string | हाँ |  |
| userIds | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserPresenceStatusesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserPresenceStatuses उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto urlIdWS = U("article-789");
auto userIds = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> optionalFilter = boost::none;
api->getUserPresenceStatuses(tenantId, urlIdWS, userIds)
    .then([](pplx::task<std::shared_ptr<GetUserPresenceStatusesResponse>> t){
        try{
            auto response = t.get();
        }catch(...){
        }
    });
[inline-code-end]