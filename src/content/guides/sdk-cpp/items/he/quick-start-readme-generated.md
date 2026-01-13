### שימוש ב-APIs מאומתים (DefaultAPI)

**חשוב:**
1. עליך להגדיר את כתובת הבסיס (הגנרטור cpp-restsdk לא קורא אותה מה-OpenAPI spec)
2. עליך להגדיר את מפתח ה-API שלך על ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה כן, הבקשות ייכשלו עם שגיאת 401.

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

    // עכשיו בצע קריאות API מאומתות
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

### בעיות נפוצות

1. **שגיאת "URI חייב להכיל שם מארח"**: וודא שאתה קורא ל-`config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` לפני יצירת ה-ApiClient. הגנרטור cpp-restsdk אינו קורא אוטומטית את כתובת השרת מהמפרט של OpenAPI.
2. **שגיאת 401 "missing-api-key"**: וודא שאתה קורא ל-`config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` לפני יצירת מופע ה-DefaultAPI.
3. **מחלקת API שגויה**: השתמש ב-`DefaultAPI` עבור בקשות מאומתות בצד השרת, וב-`PublicAPI` עבור בקשות בצד הלקוח/ציבוריות.