כל שיטות ה-API ב-SDK הזה מחזירות `pplx::task<std::shared_ptr<ResponseType>>` מה-C++ REST SDK. זה נותן לך גמישות באופן הטיפול בתגובות ה-API.

### קריאות סינכרוניות עם `.get()`

השתמש ב-`.get()` כדי לחסום את ה-thread הקורא עד להשלמת הבקשה ולקבל את התוצאה באופן סינכרוני:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// פרמטרים נדרשים מועברים לפי מיקום; אופציונליים נכנסים למבנה ה-options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// קריאה ל-.get() כדי לחסום ולקבל את התוצאה באופן סינכרוני
auto response = api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).get();  // חוסם עד להשלמת בקשת ה-HTTP

if (response && response->comments) {
    std::cout << "Found " << response->comments->size() << " comments" << std::endl;
}
```

### קריאות אסינכרוניות עם `.then()`

השתמש ב-`.then()` לביצוע אסינכרוני לא חוסם עם callbacks:

```cpp
auto config = std::make_shared<org::openapitools::client::api::ApiConfiguration>();
config->setBaseUrl(utility::conversions::to_string_t("https://fastcomments.com"));
config->setApiKey(utility::conversions::to_string_t("api_key"),
                  utility::conversions::to_string_t("YOUR_API_KEY"));

auto apiClient = std::make_shared<org::openapitools::client::api::ApiClient>(config);
org::openapitools::client::api::DefaultApi api(apiClient);

// פרמטרים נדרשים מועברים לפי מיקום; אופציונליים נכנסים למבנה ה-options
org::openapitools::client::api::GetCommentsOptions options;
options.urlId = utility::conversions::to_string_t("your-url-id");

// שימוש ב-.then() לביצוע אסינכרוני מבוסס callback
api.getComments(
    utility::conversions::to_string_t("your-tenant-id"),
    options
).then([](std::shared_ptr<GetComments_200_response> response) {
    // זה רץ באופן אסינכרוני כאשר הבקשה מושלמת
    if (response && response->comments) {
        std::cout << "Found " << response->comments->size() << " comments" << std::endl;
    }
});

// הביצוע ממשיך מיד ללא חסימה
std::cout << "Request sent, continuing..." << std::endl;
```

### בחירה בין סינכרוני לאסינכרוני

הבחירה תלויה בסביבת הריצה שלך ובארכיטקטורת היישום:

**`.get()` (חסימה סינכרונית)**
- חוסם את ה-thread הקורא עד להשלמת בקשת ה-HTTP
- זרימת קוד פשוטה יותר, קלה יותר להבנה
- מתאים ל-worker threads ייעודיים, עיבוד אצווה, או כלי שורת פקודה
- **לא מתאים** ל-event loops, threads של GUI, או שרתים חד-תהלוכתיים

**`.then()` (אסינכרוני לא חוסם)**
- מחזיר מיד, ה-callback מבוצע כאשר הבקשה מושלמת
- אינו חוסם את ה-thread הקורא
- נדרש לארכיטקטורות מבוססות אירועים, יישומי GUI, או event loops חד-תהלוכתיים
- מאפשר שרשור של מספר פעולות
- זרימת בקרה מורכבת יותר

סוויטת הבדיקות של ה-SDK משתמשת ב-`.get()` באופן בלעדי, אך זה מתאים לסביבת הבדיקה שבה חסימה מקובלת.