## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const UnFlagCommentOptions& | Yes |  |

## Απόκριση

Επιστρέφει: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα unFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = UnFlagCommentOptions{};
options.reason = boost::optional<utility::string_t>(U("Resolved by moderator"));
api->unFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<FlagCommentResponse> response) {
        if (response) {
            auto status = response->status;
            // επεξεργασία κατάστασης εάν χρειάζεται
        }
    })
    .then([](pplx::task<void> previous) {
        try {
            previous.get();
        } catch (const std::exception& e) {
            // διαχείριση σφάλματος
        }
    });
[inline-code-end]