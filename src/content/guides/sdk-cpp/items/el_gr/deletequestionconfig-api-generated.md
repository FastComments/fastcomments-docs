## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteQuestionConfig Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto configId = utility::conversions::to_string_t("question-config-456");

api->deleteQuestionConfig(tenantId, configId)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // χειρισμός επιτυχούς διαγραφής
    })
    .then([](pplx::task<void> t) {
        try {
            t.get();
        } catch (const std::exception&) {
            // χειρισμός σφάλματος
        }
    });
[inline-code-end]