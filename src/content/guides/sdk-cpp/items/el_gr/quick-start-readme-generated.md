### Χρήση Αυθεντικοποιημένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε το βασικό URL (ο γεννήτορας cpp-restsdk δεν το διαβάζει από την προδιαγραφή OpenAPI)
2. Πρέπει να ορίσετε το API key σας στο ApiClient πριν πραγματοποιήσετε αιτήματα με αυθεντικοποίηση. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το βασικό URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το API key σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Τώρα πραγματοποιήστε κλήσεις API με αυθεντικοποίηση
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

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το βασικό URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Κάντε δημόσιες κλήσεις API
    return 0;
}
```

### Χρήση API Επιτήρησης (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου των διαχειριστών. Κάθε μέθοδος δέχεται παράμετρο `sso` ώστε η κλήση να εκτελείται εκ μέρους ενός διαχειριστή αυθεντικοποιημένου μέσω SSO (δείτε την ενότητα SSO παρακάτω για το πώς να δημιουργήσετε ένα token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το βασικό URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Δώστε το SSO token του διαχειριστή για να αυθεντικοποιηθεί η κλήση
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

### Συνήθη Προβλήματα

1. **"URI must contain a hostname" error**: Βεβαιωθείτε ότι καλείτε `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε το ApiClient. Ο γεννήτορας cpp-restsdk δεν διαβάζει αυτόματα το URL του server από την προδιαγραφή OpenAPI.
2. **401 "missing-api-key" error**: Βεβαιωθείτε ότι καλείτε `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε το instance του DefaultAPI.
3. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για αιτήματα με αυθεντικοποίηση από την πλευρά του server, `PublicApi` για client-side/δημόσια αιτήματα, και `ModerationApi` για αιτήματα του πίνακα ελέγχου διαχειριστή (αυθεντικοποιημένα με SSO token διαχειριστή).
---