All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. This gives you flexibility in how you handle API responses.

### Συγχρονικές κλήσεις με `.get()`

Χρησιμοποιήστε το `.get()` για να μπλοκάρετε το νήμα που καλεί μέχρι να ολοκληρωθεί το αίτημα και να λάβετε το αποτέλεσμα συγχρονικά:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // Μπλοκάρει μέχρι να ολοκληρωθεί το αίτημα HTTP

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Ασύγχρονες κλήσεις με `.then()`

Χρησιμοποιήστε το `.then()` για μη-μπλοκαριστική ασύγχρονη εκτέλεση με callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Required parameters are positional; optional ones go in the options struct
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Αυτό εκτελείται ασύγχρονα όταν ολοκληρωθεί το αίτημα
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Επιλογή μεταξύ Συγχρονισμού και Ασύγχρονης Εκτέλεσης

Η επιλογή εξαρτάται από το περιβάλλον εκτέλεσης και την αρχιτεκτονική της εφαρμογής σας:

**`.get()` (Synchronous blocking)**
- Μπλοκάρει το νήμα που καλεί μέχρι να ολοκληρωθεί το αίτημα HTTP
- Απλούστερη ροή κώδικα, πιο εύκολο να λογιστεί κανείς
- Κατάλληλο για αποκλειστικά νήματα εργασίας, επεξεργασία παρτίδων ή εργαλεία γραμμής εντολών
- **Μη κατάλληλο** για βρόχους γεγονότων, νήματα GUI ή εξυπηρετητές μονής λειτουργίας

**`.then()` (Asynchronous non-blocking)**
- Επιστρέφει άμεσα, η callback εκτελείται όταν ολοκληρωθεί το αίτημα
- Δεν μπλοκάρει το νήμα που καλεί
- Απαιτείται για αρχιτεκτονικές βάσει γεγονότων, εφαρμογές GUI ή βρόχους γεγονότων μονής λειτουργίας
- Επιτρέπει την αθροίωση πολλαπλών λειτουργιών
- Πιο σύνθετη ροή ελέγχου

Η σειρά δοκιμών του SDK χρησιμοποιεί αποκλειστικά το `.get()`, αλλά αυτό είναι κατάλληλο για το περιβάλλον δοκιμών όπου η μπλοκάρισμα είναι αποδεκτή.