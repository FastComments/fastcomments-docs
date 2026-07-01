### Χρήση Επικυρωμένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε τη βασική URL (ο δημιουργός cpp-restsdk δεν την διαβάζει από το OpenAPI spec)
2. Πρέπει να ορίσετε το κλειδί API στο ApiClient πριν κάνετε επικυρωμένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε τη βασική URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // Ή: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε το κλειδί API σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Τώρα κάντε επικυρωμένες κλήσεις API
    return 0;
}
```

### Χρήση Δημόσιων API (PublicAPI)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε τη βασική URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Κάντε δημόσιες κλήσεις API
    return 0;
}
```

### Χρήση API Συντονισμού (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου των συντονιστών. Κάθε μέθοδος δέχεται μια παράμετρο `sso` ώστε η κλήση να εκτελείται εκ μέρους ενός συντονιστή που έχει επικυρωθεί μέσω SSO (δείτε την ενότητα SSO παρακάτω για το πώς να δημιουργήσετε ένα token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε τη βασική URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Δώστε το SSO token του συντονιστή για να επικυρώσετε την κλήση
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Συνηθισμένα Προβλήματα

1. **Σφάλμα "URI must contain a hostname"**: Βεβαιωθείτε ότι καλείτε `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε το ApiClient. Ο δημιουργός cpp-restsdk δεν διαβάζει αυτόματα τη διεύθυνση του διακομιστή από το OpenAPI spec.  
2. **Σφάλμα 401 "missing-api-key"**: Βεβαιωθείτε ότι καλείτε `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε την παρουσία DefaultAPI.  
3. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα επικυρωμένα από τον διακομιστή, `PublicApi` για αιτήματα από την πλευρά του πελάτη/δημόσια, και `ModerationApi` για αιτήματα του πίνακα ελέγχου συντονιστών (επικυρωμένα με token SSO συντονιστή).