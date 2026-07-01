### Χρήση Πιστοποιημένων API (DefaultAPI)

**Σημαντικό:**
1. Πρέπει να ορίσετε τη βασική διεύθυνση URL (ο δημιουργός cpp‑restsdk δεν τη διαβάζει από το OpenAPI spec)
2. Πρέπει να ορίσετε το κλειδί API σας στο ApiClient πριν κάνετε κλήσεις που απαιτούν πιστοποίηση. Εάν δεν το κάνετε, οι κλήσεις θα αποτύχουν με σφάλμα 401.

```cpp
#include <iostream>
#include "FastCommentsClient/api/DefaultApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΙΤΑΙ: Ορίστε τη βασική διεύθυνση URL (επιλέξτε την περιοχή σας)
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));  // US
    // Ή: config->setBaseUrl(utility::conversions::to_string_t("https://eu.fastcomments.com"));  // EU

    // ΑΠΑΙΤΙΤΑΙ: Ορίστε το κλειδί API σας
    config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_API_KEY_HERE"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::DefaultApi api(apiClient);

    // Τώρα κάντε κλήσεις API με πιστοποίηση
    return 0;
}
```

### Χρήση Δημόσιων API (PublicAPI)

Τα δημόσια σημεία τέλους δεν απαιτούν πιστοποίηση:

```cpp
#include <iostream>
#include "FastCommentsClient/api/PublicApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΙΤΑΙ: Ορίστε τη βασική διεύθυνση URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::PublicApi publicApi(apiClient);

    // Κάντε δημόσιες κλήσεις API
    return 0;
}
```

### Χρήση API Συντονισμού (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου του συντονιστή. Κάθε μέθοδος δέχεται μια παράμετρο `sso` ώστε η κλήση να εκτελείται εκ μέρους ενός συντονιστή που έχει πιστοποιηθεί μέσω SSO (δείτε την ενότητα SSO παρακάτω για το πώς να δημιουργήσετε ένα διακριτικό):

```cpp
#include <iostream>
#include "FastCommentsClient/api/ModerationApi.h"
#include "FastCommentsClient/ApiClient.h"
#include "FastCommentsClient/ApiConfiguration.h"

int main() {
    auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();

    // ΑΠΑΙΤΙΤΑΙ: Ορίστε τη βασική διεύθυνση URL
    config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));

    auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
    org::openapitools::client::api::ModerationApi moderationApi(apiClient);

    // Πραγματοποιήστε κλήση με το SSO διακριτικό του συντονιστή για πιστοποίηση
    auto ssoToken = utility::conversions::to_string_t("YOUR_MODERATOR_SSO_TOKEN");

    org::openapitools::client::api::GetCountOptions options;
    options.sso = ssoToken;

    auto response = moderationApi.getCount(options).get();

    return 0;
}
```

### Κοινά Προβλήματα

1. **Σφάλμα “URI must contain a hostname”**: Βεβαιωθείτε ότι έχετε καλέσει `config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"))` πριν δημιουργήσετε το ApiClient. Ο δημιουργός cpp‑restsdk δεν διαβάζει αυτόματα τη διεύθυνση του server από το OpenAPI spec.
2. **Σφάλμα 401 “missing‑api‑key”**: Βεβαιωθείτε ότι έχετε καλέσει `config->setApiKey(utility::conversions::to_string_t("api_key"), utility::conversions::to_string_t("YOUR_KEY"))` πριν δημιουργήσετε την περίπτωση DefaultAPI.
3. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για κλήσεις εξυπηρετητή με πιστοποίηση, `PublicApi` για κλήσεις πελάτη/δημόσιες, και `ModerationApi` για κλήσεις πίνακα ελέγχου συντονιστή (πιστοποιημένες με διακριτικό SSO του συντονιστή).