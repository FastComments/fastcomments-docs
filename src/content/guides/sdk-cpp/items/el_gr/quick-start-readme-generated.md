### Χρήση Πιστοποιημένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε τη βασική διεύθυνση URL (ο cpp-restsdk generator δεν τη διαβάζει από το OpenAPI spec)
2. Πρέπει να ορίσετε το API key σας στο ApiClient πριν κάνετε αιτήσεις που απαιτούν αυθεντικοποίηση. Αν δεν το κάνετε, οι αιτήσεις θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε τη βασική διεύθυνση URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το API key σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Now make authenticated API calls
    return 0;
}
```

### Χρήση Δημόσιων API (PublicAPI)

Οι δημόσιες endpoints δεν απαιτούν αυθεντικοποίηση:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε τη βασική διεύθυνση URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Κάντε δημόσιες κλήσεις API
    return 0;
}
```

### Συχνά προβλήματα

1. **Σφάλμα "URI must contain a hostname"**: Βεβαιωθείτε ότι καλείτε `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε το ApiClient. Ο cpp-restsdk generator δεν διαβάζει αυτόματα το server URL από το OpenAPI spec.
2. **Σφάλμα 401 "missing-api-key"**: Βεβαιωθείτε ότι καλείτε `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε το αντικείμενο DefaultAPI.
3. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultAPI` για server-side αιτήσεις με αυθεντικοποίηση, `PublicAPI` για client-side/δημόσιες αιτήσεις.