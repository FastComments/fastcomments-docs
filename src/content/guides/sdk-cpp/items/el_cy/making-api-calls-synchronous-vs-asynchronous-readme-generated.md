Όλες οι μέθοδοι API σε αυτό το SDK επιστρέφουν `pplx::task<std::shared_ptr<ResponseType>>` από το C++ REST SDK. Αυτό σας δίνει ευελιξία στον τρόπο που χειρίζεστε τις αποκρίσεις API.

### Σύνθετες κλήσεις με `.get()`

Χρησιμοποιήστε το `.get()` για να μπλοκάρετε το νήμα που καλεί τη μέθοδο μέχρι να ολοκληρωθεί το αίτημα και να λάβετε το αποτέλεσμα συγχρονισμένα:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Οι απαιτούμενες παράμετροι είναι θέσεων· οι προαιρετικές πηγαίνουν στη δομή επιλογών
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Κλήση .get() για να μπλοκάρει και να λάβει το αποτέλεσμα συγχρονισμένα
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Μπλοκάρει μέχρι να ολοκληρωθεί το HTTP αίτημα

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Ασύγχρονες κλήσεις με `.then()`

Χρησιμοποιήστε το `.then()` για μη‑μπλοκαριστική ασύγχρονη εκτέλεση με callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Οι απαιτούμενες παράμετροι είναι θέσεων· οι προαιρετικές πηγαίνουν στη δομή επιλογών
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Χρησιμοποιήστε .then() για εκτέλεση με βάση callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Αυτό εκτελείται ασύγχρονα όταν ολοκληρωθεί το αίτημα
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Η εκτέλεση συνεχίζεται άμεσα χωρίς μπλοκάρισμα
std::cout << "Request sent, continuing..." << std::endl;
```

### Επιλογή μεταξύ Συγχρονικής και Ασύγχρονης

Η επιλογή εξαρτάται από το περιβάλλον εκτέλεσής σας και την αρχιτεκτονική της εφαρμογής:

**`.get()` (Συγχρονικό μπλοκάρισμα)**
- Μπλοκάρει το νήμα που καλεί τη μέθοδο μέχρι να ολοκληρωθεί το HTTP αίτημα
- Πιο απλή ροή κώδικα, πιο εύκολο στην κατανόηση
- Κατάλληλο για αποκλειστικά νήματα εργατών, batch επεξεργασία ή εργαλεία γραμμής εντολών
- **Μη κατάλληλο** για βρόχους συμβάντων, νήματα GUI ή μονονηματικούς διακομιστές

**`.then()` (Ασύγχρονο μη‑μπλοκάρισμα)**
- Επιστρέφει αμέσως, το callback εκτελείται όταν ολοκληρωθεί το αίτημα
- Δεν μπλοκάρει το νήμα που καλεί τη μέθοδο
- Απαιτείται για αρχιτεκτονικές γεγονότων, GUI εφαρμογές ή μονονηματικούς βρόχους συμβάντων
- Επιτρέπει αλυσίδωση πολλαπλών λειτουργιών
- Πιο πολύπλοκη ροή ελέγχου

Η ομάδα δοκιμών του SDK χρησιμοποιεί αποκλειστικά `.get()`, αλλά αυτό είναι κατάλληλο για το περιβάλλον δοκιμών όπου το μπλοκάρισμα είναι αποδεκτό.