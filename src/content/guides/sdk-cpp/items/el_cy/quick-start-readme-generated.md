### Χρήση Πιστοποιημένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε το base URL (ο γεννήτορας cpp-restsdk δεν το διαβάζει από το OpenAPI spec)
2. Πρέπει να ορίσετε το API key σας στο ApiClient πριν κάνετε πιστοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το base URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // OR: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το API key σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Τώρα πραγματοποιήστε πιστοποιημένες κλήσεις στο API
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

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Εκτελέστε δημόσιες κλήσεις API
    return 0;
}
```

### Χρήση Moderation APIs (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου των διαχειριστών. Κάθε μέθοδος δέχεται παράμετρο `sso`, έτσι ώστε η κλήση να εκτελείται εκ μέρους ενός διαχειριστή που έχει αυθεντικοποιηθεί με SSO (δείτε την ενότητα SSO παρακάτω για το πώς να δημιουργήσετε ένα token):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το base URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Περάστε το SSO token του διαχειριστή για να αυθεντικοποιήσετε την κλήση
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

### Συνηθισμένα Προβλήματα

1. **"URI must contain a hostname" error**: Βεβαιωθείτε ότι καλείτε `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε τον ApiClient. Ο γεννήτορας cpp-restsdk δεν διαβάζει αυτόματα το server URL από το OpenAPI spec.
2. **401 "missing-api-key" error**: Βεβαιωθείτε ότι καλείτε `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε το παράδειγμα DefaultAPI.
3. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για server-side πιστοποιημένα αιτήματα, `PublicApi` για client-side/δημόσια αιτήματα, και `ModerationApi` για αιτήματα του πίνακα ελέγχου διαχειριστή (πιστοποιημένα με SSO token διαχειριστή).