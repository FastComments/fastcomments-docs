### שימוש ב-API מאומתים (DefaultAPI)

**חשוב:**
1. עליך להגדיר את כתובת הבסיס (המחולל cpp-restsdk אינו קורא אותה ממפרט ה-OpenAPI)
2. עליך להגדיר את מפתח ה-API שלך ב-ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יכשלו עם שגיאת 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // נדרש: הגדר את כתובת הבסיס (בחר את האזור שלך)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // או: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // נדרש: הגדר את מפתח ה-API שלך
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // עכשיו בצע קריאות API מאומתות
    return 0;
}
```

### שימוש ב-API ציבוריים (PublicAPI)

קצות קצה ציבוריים אינם דורשים אימות:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // נדרש: הגדר את כתובת הבסיס
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // בצע קריאות API ציבוריות
    return 0;
}
```

### שימוש ב-API של מודרציה (ModerationApi)

`ModerationApi` מחזק את לוח המחוונים של המודרטור. כל שיטה מקבלת פרמטר `sso` כך שהקריאה מתבצעת בשם מודרטור שמאומת באמצעות SSO (ראו את סעיף ה-SSO למטה כיצד ליצור טוקן):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // נדרש: הגדר את כתובת הבסיס
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // העבר את טוקן ה-SSO של המודרטור כדי לאמת את הקריאה
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### בעיות נפוצות

1. **שגיאת "URI must contain a hostname"**: ודא שאתה קורא ל-`config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` לפני יצירת ApiClient. המחולל cpp-restsdk אינו קורא אוטומטית את כתובת השרת ממפרט ה-OpenAPI.
2. **שגיאת 401 "missing-api-key"**: ודא שאתה קורא ל-`config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` לפני יצירת המופע של DefaultAPI.
3. **מחלקת API לא נכונה**: השתמש ב-`DefaultApi` לבקשות מאומתות בצד השרת, ב-`PublicApi` לבקשות בצד הלקוח/ציבוריות, וב-`ModerationApi` לבקשות לוח המחוונים של המודרטור (מאומתות באמצעות טוקן SSO של מודרטור).