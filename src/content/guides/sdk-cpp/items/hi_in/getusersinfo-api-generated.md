किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उन यूज़र्स को समृद्ध करने के लिए उपयोग किया जाता है जो अभी-अभी एक presence इवेंट के माध्यम से दिखाई दिए।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दी जाती हैं)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| ids | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUsersInfo उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // प्रतिक्रिया प्रोसेस करें
    }catch(const std::exception&){
        // त्रुटि को संभालें
    }
});
[inline-code-end]