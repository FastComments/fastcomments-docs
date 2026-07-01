List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Απαιτεί το `enableFChat` να είναι true στη διαμορφωμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.  
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetPagesPublicOptions& | Yes |  |

## Απάντηση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // επεξεργασία της απάντησης αν χρειάζεται
    }catch(const std::exception&){
        // διαχείριση σφάλματος αν χρειάζεται
    }
});
[inline-code-end]