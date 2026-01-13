All API methods in this SDK return `pplx::task<std::shared_ptr<ResponseType>>` from the C++ REST SDK. Αυτό σας δίνει ευελιξία στον τρόπο που χειρίζεστε τις απαντήσεις του API.

### Συγχρονικές κλήσεις με `.get()`

Χρησιμοποιήστε `.get()` για να μπλοκάρετε το νήμα που καλεί μέχρι να ολοκληρωθεί το αίτημα και να λάβετε το αποτέλεσμα συγχρονικά:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Call .get() to block and get the result synchronously
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none,  // page
    boost::none,  // limit
    boost::none,  // skip
    boost::none,  // asTree
    boost::none,  // skipChildren
    boost::none,  // limitChildren
    boost::none,  // maxTreeDepth
    utility::conversions::to_string_t("your-url-id"),  // urlId
    boost::none,  // userId
    boost::none,  // anonUserId
    boost::none,  // contextUserId
    boost::none,  // hashTag
    boost::none,  // parentId
    boost::none   // direction
).get();  // Blocks until the HTTP request completes

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Ασύγχρονες κλήσεις με `.then()`

Χρησιμοποιήστε `.then()` για μη αποκλειστική ασύγχρονη εκτέλεση με callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Use .then() for asynchronous callback-based execution
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // This runs asynchronously when the request completes
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Execution continues immediately without blocking
std::cout << "Request sent, continuing..." << std::endl;
```

### Επιλογή μεταξύ Συγχρονικής και Ασύγχρονης

Η επιλογή εξαρτάται από το περιβάλλον εκτέλεσης και την αρχιτεκτονική της εφαρμογής σας:

**`.get()` (Συγχρονικό — μπλοκάρισμα)**
- Μπλοκάρει το νήμα που καλεί μέχρι να ολοκληρωθεί το HTTP αίτημα
- Απλούστερη ροή κώδικα, ευκολότερη στην κατανόηση
- Κατάλληλο για αφιερωμένα νήματα εργασίας, επεξεργασία παρτίδων ή εργαλεία γραμμής εντολών
- **Μη κατάλληλο** για βρόχους συμβάντων, νήματα GUI ή μονονηματικούς διακομιστές

**`.then()` (Ασύγχρονο — μη αποκλειστικό)**
- Επιστρέφει αμέσως, η callback εκτελείται όταν ολοκληρωθεί το αίτημα
- Δεν μπλοκάρει το νήμα που καλεί
- Απαραίτητο για αρχιτεκτονικές καθοδηγούμενες από συμβάντα, εφαρμογές GUI ή μονονηματικούς βρόχους συμβάντων
- Επιτρέπει την αλυσιδωτή εκτέλεση πολλαπλών λειτουργιών
- Πιο πολύπλοκη ροή ελέγχου

Το σύνολο δοκιμών του SDK χρησιμοποιεί αποκλειστικά `.get()`, αλλά αυτό είναι κατάλληλο για το περιβάλλον δοκιμών όπου το μπλοκάρισμα είναι αποδεκτό.