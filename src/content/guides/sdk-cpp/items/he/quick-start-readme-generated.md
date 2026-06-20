### שימוש ב-APIs מאומתים (DefaultAPI)

**Important:**
1. עליך להגדיר את כתובת הבסיס (הגנרטור cpp-restsdk לא קורא אותה מתוך מפרט ה-OpenAPI)
2. עליך להגדיר את מפתח ה-API ב-ApiClient לפני ביצוע קריאות מאומתות. אם לא תעשה זאת, הקריאות ייכשלו עם שגיאת 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // דרוש: הגדר את כתובת הבסיס (בחר את האזור שלך)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // ארה"ב
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // אירופה

    // דרוש: הגדר את מפתח ה-API שלך
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### שימוש ב-APIs ציבוריים (PublicAPI)

נקודות קצה ציבוריות אינן דורשות אימות:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // דרוש: הגדר את כתובת הבסיס
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // בצע קריאות API ציבוריות
    return 0;
}
```

### שימוש ב-APIs למודרציה (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Every method accepts an `sso` parameter so the call runs on behalf of an SSO-authenticated moderator (see the SSO section below
for how to create a token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // דרוש: הגדר את כתובת הבסיס
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // העבר את טוקן ה-SSO של המודרטור כדי לאמת את הקריאה
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    auto response = moderationApi.getCount(
        boost::none,  // textSearch
        boost::none,  // byIPFromComment
        boost::none,  // filter
        boost::none,  // searchFilters
        boost::none,  // demo
        ssoToken      // sso
    ).get();

    return 0;
}
```

### בעיות נפוצות

1. **"URI must contain a hostname" error**: ודא שקראת ל-`config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` לפני יצירת ה-ApiClient. הגנרטור cpp-restsdk לא קורא באופן אוטומטי את כתובת השרת מתוך מפרט ה-OpenAPI.
2. **401 "missing-api-key" error**: ודא שקראת ל-`config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` לפני יצירת מופע DefaultAPI.
3. **Wrong API class**: השתמש ב-`DefaultApi` עבור קריאות מאומתות בצד השרת, ב-`PublicApi` עבור קריאות צד-לקוח/ציבוריות, וב-`ModerationApi` עבור קריאות ללוח הבקרה של המודרטור (מאומתות עם טוקן SSO של מודרטור).
---