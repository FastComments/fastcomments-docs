किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए रिज़ॉल्व किए गए कस्टम कॉन्फ़िग पर `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेजों को अनुरोध करने वाले उपयोगकर्ता के ग्रुप एक्सेस के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetPagesPublicOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // आवश्यक हो तो प्रतिक्रिया प्रोसेस करें
    }catch(const std::exception&){
        // आवश्यक हो तो त्रुटि संभालें
    }
});
[inline-code-end]