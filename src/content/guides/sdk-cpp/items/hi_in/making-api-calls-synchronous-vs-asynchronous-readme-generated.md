इस SDK के सभी API मेथड्स C++ REST SDK से `pplx::task<std::shared_ptr<ResponseType>>` रिटर्न करते हैं। इससे आपको API रिस्पॉन्स को हैंडल करने के तरीके में लचीलापन मिलता है।

### `.get()` के साथ सिंक्रोनस कॉल्स

कॉलिंग थ्रेड को ब्लॉक करने और रिज़ल्ट को सिंक्रोनस रूप से प्राप्त करने के लिए `.get()` का उपयोग करें, जब तक कि रिक्वेस्ट पूरी न हो जाए:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// आवश्यक पैरामीटर पोज़िशनल होते हैं; वैकल्पिक पैरामीटर options स्ट्रक्चर में जाते हैं
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// ब्लॉक करने और रिज़ल्ट को सिंक्रोनस रूप से प्राप्त करने के लिए .get() कॉल करें
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // HTTP रिक्वेस्ट पूरा होने तक ब्लॉक करता है

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### `.then()` के साथ एसिंक्रोनस कॉल्स

कॉलबैक के साथ नॉन-ब्लॉकिंग एसिंक्रोनस एक्ज़ीक्यूशन के लिए `.then()` का उपयोग करें:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// आवश्यक पैरामीटर पोज़िशनल होते हैं; वैकल्पिक पैरामीटर options स्ट्रक्चर में जाते हैं
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// कॉलबैक-आधारित एसिंक्रोनस एक्ज़ीक्यूशन के लिए .then() का उपयोग करें
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // रिक्वेस्ट पूरा होने पर यह एसिंक्रोनस रूप से चलता है
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// एक्ज़ीक्यूशन बिना ब्लॉक किए तुरंत आगे बढ़ जाता है
std::cout << "Request sent, continuing..." << std::endl;
```

### सिंक्रोनस और एसिंक्रोनस में से चुनाव

चुनाव आपके रनटाइम एनवायरनमेंट और एप्लिकेशन आर्किटेक्चर पर निर्भर करता है:

**`.get()` (सिंक्रोनस ब्लॉकिंग)**
- HTTP रिक्वेस्ट पूरा होने तक कॉलिंग थ्रेड को ब्लॉक करता है
- सरल कोड फ्लो, समझने में आसान
- समर्पित वर्कर थ्रेड्स, बैच प्रोसेसिंग, या कमांड-लाइन टूल्स के लिए उपयुक्त
- इवेंट लूप्स, GUI थ्रेड्स, या सिंगल-थ्रेडेड सर्वर के लिए **उपयुक्त नहीं**

**`.then()` (एसिंक्रोनस नॉन-ब्लॉकिंग)**
- तुरंत रिटर्न करता है, रिक्वेस्ट पूरा होने पर कॉलबैक एक्ज़ीक्यूट होता है
- कॉलिंग थ्रेड को ब्लॉक नहीं करता
- इवेंट-ड्रिवन आर्किटेक्चर, GUI एप्लिकेशन, या सिंगल-थ्रेडेड इवेंट लूप्स के लिए आवश्यक
- कई ऑपरेशन्स को चेन करने की अनुमति देता है
- अधिक जटिल कंट्रोल फ्लो

SDK का टेस्ट सूट केवल `.get()` का उपयोग करता है, लेकिन यह टेस्ट एनवायरनमेंट के लिए उपयुक्त है जहाँ ब्लॉकिंग स्वीकार्य है।