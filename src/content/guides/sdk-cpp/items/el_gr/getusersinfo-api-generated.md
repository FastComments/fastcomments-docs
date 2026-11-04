Μαζικές πληροφορίες χρηστών για έναν tenant. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από το User / SSOUser.
Χρησιμοποιείται από το widget σχολίων για τον εμπλουτισμό χρηστών που μόλις εμφανίστηκαν μέσω ενός presence event.
Χωρίς πλαίσιο σελίδας: το απόρρητο επιβάλλεται ομοιόμορφα (τα ιδιωτικά προφίλ καλύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| ids | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // επεξεργασία απάντησης
    }catch(const std::exception&){
        // διαχείριση σφάλματος
    }
});
[inline-code-end]