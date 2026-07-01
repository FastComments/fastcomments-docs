Skup informacija o korisnicima za najamnika. Dati userIds, vraća informacije za prikaz iz User / SSOUser.  
Koristi se u vidžetu komentara za obogaćivanje korisnika koji su upravo pojavili putem događaja prisustva.  
Bez konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // obradi odgovor
    }catch(const std::exception&){
        // obradi grešku
    }
});
[inline-code-end]