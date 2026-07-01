## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |
| options | const FlagCommentOptions& | Ναι |  |

## Απάντηση

Επιστρέφει: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα flagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<FlagCommentOptions>();
opts->reason = utility::conversions::to_string_t("spam");
opts->note = boost::optional<utility::string_t>(utility::conversions::to_string_t("User posted duplicate links"));

api->flagComment(utility::conversions::to_string_t("my-tenant-123"),
                 utility::conversions::to_string_t("comment-456"),
                 *opts)
    .then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
        auto resp = t.get();
    });
[inline-code-end]