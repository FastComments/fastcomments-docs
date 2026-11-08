पेज पर उन पिछले टिप्पणीकारों को जो अभी ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online को पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर afterName से आगे $gt के माध्यम से आंशिक {tenantId, urlId, commenterName}
इंडेक्स को ट्रैवर्स करता है, बिना $skip लागत के।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| options | const GetOfflineUsersOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getOfflineUsers उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]