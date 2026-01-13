Όλες οι μεθόδοι API σε αυτό το SDK επιστρέφουν `pplx::task<std::shared_ptr<ResponseType>>` από το C++ REST SDK. Αυτό σας δίνει ευελιξία στον τρόπο που χειρίζεστε τις απαντήσεις του API.

### Συγχρονικές κλήσεις με `.get()`

Χρησιμοποιήστε `.get()` για να αποκλείσετε το καλούν νήμα μέχρι να ολοκληρωθεί το αίτημα και να ανακτήσετε το αποτέλεσμα συγχρονικά:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Καλέστε .get() για να μπλοκάρετε και να λάβετε το αποτέλεσμα συγχρονικά
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
).get();  // Μπλοκάρει μέχρι να ολοκληρωθεί το αίτημα HTTP

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Ασύγχρονες κλήσεις με `.then()`

Χρησιμοποιήστε `.then()` για ασύγχρονη εκτέλεση χωρίς μπλοκάρισμα με callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// Χρησιμοποιήστε .then() για ασύγχρονη εκτέλεση με callbacks
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // Αυτό τρέχει ασύγχρονα όταν το αίτημα ολοκληρωθεί
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// Η εκτέλεση συνεχίζεται αμέσως χωρίς μπλοκάρισμα
std::cout << "Request sent, continuing..." << std::endl;
```

### Επιλογή μεταξύ Συγχρονικού και Ασύγχρονου

Η επιλογή εξαρτάται από το περιβάλλον εκτέλεσης και την αρχιτεκτονική της εφαρμογής σας:

**`.get()` (Συγχρονικό, μπλοκάρισμα)**
- Blocks the calling thread until the HTTP request completes
- Απλούστερη ροή κώδικα, πιο εύκολο στην κατανόηση
- Κατάλληλο για αφιερωμένα worker threads, batch επεξεργασία ή εργαλεία γραμμής εντολών
- **Δεν είναι κατάλληλο** για event loops, GUI threads ή single-threaded servers

**`.then()` (Ασύγχρονο, χωρίς μπλοκάρισμα)**
- Επιστρέφει άμεσα, το callback εκτελείται όταν το αίτημα ολοκληρωθεί
- Δεν μπλοκάρει το καλούν νήμα
- Απαραίτητο για event-driven αρχιτεκτονικές, GUI εφαρμογές ή single-threaded event loops
- Επιτρέπει τη σύνδεση πολλαπλών λειτουργιών σε αλυσίδα
- Πιο πολύπλοκη ροή ελέγχου

Το test suite του SDK χρησιμοποιεί αποκλειστικά `.get()`, αλλά αυτό είναι κατάλληλο για το περιβάλλον δοκιμών όπου το μπλοκάρισμα είναι αποδεκτό.