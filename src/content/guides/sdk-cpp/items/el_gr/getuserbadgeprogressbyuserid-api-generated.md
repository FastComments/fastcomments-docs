## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| userId | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadgeProgressByUserId'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<std::shared_ptr<APIGetUserBadgeProgressResponse>> responseOpt;
api->getUserBadgeProgressByUserId(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com"))
    .then([&responseOpt](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) {
        try {
            responseOpt = t.get();
        } catch (...) {
            responseOpt = boost::none;
        }
    });
[inline-code-end]