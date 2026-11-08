## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| userId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeleteSubscriptionAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteSubscription उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->deleteSubscription(utility::string_t(U("my-tenant-123")), utility::string_t(U("sub-456")), boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))))
    .then([](std::shared_ptr<DeleteSubscriptionAPIResponse> resp){
        if (resp) {
        }
    });
[inline-code-end]