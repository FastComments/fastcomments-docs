### प्रमाणित APIs का उपयोग (DefaultAPI)

**महत्वपूर्ण:**
1. आपको बेस URL सेट करना होगा (cpp-restsdk जनरेटर इसे OpenAPI spec से नहीं पढ़ता)
2. प्रमाणित अनुरोध करने से पहले आपको ApiClient पर अपनी API key सेट करनी होगी। यदि आप ऐसा नहीं करते, तो अनुरोध 401 त्रुटि के साथ विफल होंगे।

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // आवश्यक: बेस URL सेट करें (अपना region चुनें)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // या: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // आवश्यक: अपनी API key सेट करें
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // अब प्रमाणित API कॉल करें
    return 0;
}
```

### सार्वजनिक APIs का उपयोग (PublicAPI)

सार्वजनिक endpoints के लिए प्रमाणीकरण की आवश्यकता नहीं है:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // आवश्यक: बेस URL सेट करें
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // सार्वजनिक API कॉल करें
    return 0;
}
```

### मॉडरेशन APIs का उपयोग (ModerationApi)

`ModerationApi` मॉडरेटर डैशबोर्ड को संचालित करता है। हर method एक `sso` parameter स्वीकार करता है ताकि कॉल एक SSO-प्रमाणित मॉडरेटर की ओर से चले (token बनाने का तरीका नीचे SSO अनुभाग देखें):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // आवश्यक: बेस URL सेट करें
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // कॉल को प्रमाणित करने के लिए मॉडरेटर का SSO token पास करें
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### सामान्य समस्याएँ

1. **"URI must contain a hostname" त्रुटि**: सुनिश्चित करें कि आपने ApiClient बनाने से पहले `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` कॉल किया है। cpp-restsdk जनरेटर OpenAPI spec से server URL स्वचालित रूप से नहीं पढ़ता।
2. **401 "missing-api-key" त्रुटि**: सुनिश्चित करें कि आपने DefaultAPI instance बनाने से पहले `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` कॉल किया है।
3. **गलत API class**: server-side प्रमाणित अनुरोधों के लिए `DefaultApi`, client-side/सार्वजनिक अनुरोधों के लिए `PublicApi`, और मॉडरेटर डैशबोर्ड अनुरोधों के लिए `ModerationApi` (मॉडरेटर SSO token से प्रमाणित) का उपयोग करें।