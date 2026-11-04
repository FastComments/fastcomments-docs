Όλες οι μέθοδοι API σε αυτό το SDK επιστρέφουν `pplx::task<std::shared_ptr<ResponseType>>` από το C++ REST SDK. Αυτό σας δίνει ευελιξία στον τρόπο χειρισμού των απαντήσεων του API.

### Σύγχρονες κλήσεις με `.get()`

Χρησιμοποιήστε το `.get()` για να μπλοκάρετε το νήμα κλήσης μέχρι να ολοκληρωθεί το αίτημα και να ανακτήσετε το αποτέλεσμα σύγχρονα:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Οι υποχρεωτικές παράμετροι είναι θέσης· οι προαιρετικές μπαίνουν στο struct options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Καλέστε .get() για να μπλοκάρει και να λάβει το αποτέλεσμα σύγχρονα
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Μπλοκάρει μέχρι να ολοκληρωθεί το HTTP αίτημα

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Ασύγχρονες κλήσεις με `.then()`

Χρησιμοποιήστε το `.then()` για μη μπλοκαρισμένη ασύγχρονη εκτέλεση με callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Οι υποχρεωτικές παράμετροι είναι θέσης· οι προαιρετικές μπαίνουν στο struct options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Χρησιμοποιήστε .then() για ασύγχρονη εκτέλεση με callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Αυτό εκτελείται ασύγχρονα όταν ολοκληρωθεί το αίτημα
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Η εκτέλεση συνεχίζεται αμέσως χωρίς μπλοκάρισμα
std::cout << "Request sent, continuing..." << std::endl;
```

### Επιλογή μεταξύ Συγχρόνου και Ασύγχρονου

Η επιλογή εξαρτάται από το περιβάλλον εκτέλεσης και την αρχιτεκτονική της εφαρμογής σας:

**`.get()` (Σύγχρονο με μπλοκάρισμα)**
- Μπλοκάρει το νήμα κλήσης μέχρι να ολοκληρωθεί το HTTP αίτημα
- Απλούστερη ροή κώδικα, πιο εύκολος συλλογισμός
- Κατάλληλο για αποκλειστικά νήματα εργασίας, μαζική επεξεργασία ή εργαλεία γραμμής εντολών
- **Δεν είναι κατάλληλο** για event loops, νήματα GUI ή μονονηματικούς servers

**`.then()` (Ασύγχρονο χωρίς μπλοκάρισμα)**
- Επιστρέφει αμέσως, το callback εκτελείται όταν ολοκληρωθεί το αίτημα
- Δεν μπλοκάρει το νήμα κλήσης
- Απαιτείται για αρχιτεκτονικές βασισμένες σε events, εφαρμογές GUI ή μονονηματικά event loops
- Επιτρέπει την αλυσιδωτή σύνδεση πολλαπλών λειτουργιών
- Πιο σύνθετη ροή ελέγχου

Η σουίτα δοκιμών του SDK χρησιμοποιεί αποκλειστικά το `.get()`, κάτι που είναι κατάλληλο για το περιβάλλον δοκιμών όπου το μπλοκάρισμα είναι αποδεκτό.