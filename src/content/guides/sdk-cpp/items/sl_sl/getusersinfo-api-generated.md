Masovni podatki o uporabniku za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.  
Uporablja se v pripomočku za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili preko dogodka prisotnosti.  
Brez konteksta strani: zasebnost je dosledno uveljavljena (zasebni profili so maskirani).

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // obdelaj odgovor
    }catch(const std::exception&){
        // obravnavaj napako
    }
});
[inline-code-end]