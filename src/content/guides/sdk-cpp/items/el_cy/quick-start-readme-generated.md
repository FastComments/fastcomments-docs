### Χρήση Επαληθευμένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε τη βασική διεύθυνση URL (cpp-restsdk generator doesn't read it from the OpenAPI spec)
2. Πρέπει να ορίσετε το κλειδί API σας στο ApiClient πριν κάνετε αιτήματα που απαιτούν έλεγχο ταυτότητας. Εάν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε τη βασική διεύθυνση URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // Ή: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // ΕΕ

    // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το κλειδί API σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Τώρα κάντε επαληθευμένες κλήσεις API
    return 0;
}
```

### Χρήση Δημόσιων API (PublicAPI)

Τα δημόσια endpoints δεν απαιτούν έλεγχο ταυτότητας:

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

### Συνηθισμένα Προβλήματα

1. **"URI must contain a hostname" error**: Βεβαιωθείτε ότι καλείτε `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε το ApiClient. Ο cpp-restsdk generator δεν διαβάζει αυτόματα τη διεύθυνση του server από το OpenAPI spec.
2. **401 "missing-api-key" error**: Βεβαιωθείτε ότι καλείτε `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε το instance του DefaultAPI.
3. **Wrong API class**: Χρησιμοποιήστε `DefaultAPI` για server-side αιτήματα που απαιτούν έλεγχο ταυτότητας, `PublicAPI` για client-side/δημόσια αιτήματα.