כל שיטות ה‑API בערכת ה‑SDK הזו מחזירות `pplx::task<std::shared_ptr<ResponseType>>` מתוך ה‑C++ REST SDK. הדבר נותן לך גמישות באופן שבו אתה מטפל בתגובות ה‑API.

### Synchronous Calls with `.get()`

Use `.get()` to block the calling thread until the request completes and retrieve the result synchronously:

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
).get();  // חוסם עד שבקשת ה‑HTTP תסתיים

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### Asynchronous Calls with `.then()`

Use `.then()` for non-blocking asynchronous execution with callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// השתמש ב‑.then() לביצוע אסינכרוני מבוסס callback
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none,
    boost::none, boost::none,
    utility::conversions::to_string_t("your-url-id"),
    boost::none, boost::none, boost::none, boost::none, boost::none, boost::none
).then([](std::shared_ptr<GetComments_200_response> response) {
    // זה רץ באופן אסינכרוני כאשר הבקשה מסתיימת
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// ההרצה נמשכת מיד בלי חסימה
std::cout << "Request sent, continuing..." << std::endl;
```

### Choosing Between Synchronous and Asynchronous

The choice depends on your runtime environment and application architecture:

**`.get()` (Synchronous blocking)**
- Blocks the calling thread until the HTTP request completes
- Simpler code flow, easier to reason about
- Suitable for dedicated worker threads, batch processing, or command-line tools
- **Not suitable** for event loops, GUI threads, or single-threaded servers

**`.then()` (Asynchronous non-blocking)**
- Returns immediately, callback executes when request completes
- Does not block the calling thread
- Required for event-driven architectures, GUI applications, or single-threaded event loops
- Allows chaining multiple operations
- More complex control flow

The SDK's test suite uses `.get()` exclusively, but this is appropriate for the test environment where blocking is acceptable.