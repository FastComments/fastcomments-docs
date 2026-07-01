Bulk user info for a tenant. Given userIds, return display info from User / SSOUser.  
Used by the comment widget to enrich users that just appeared via a presence event.  
No page context: privacy is enforced uniformly (private profiles are masked).

Μαζική πληροφορία χρηστών για έναν ενοικιαστή. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.  
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει χρήστες που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας.  
Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ είναι καλυμμένα).

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

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
        // χειρισμός σφάλματος
    }
});
[inline-code-end]