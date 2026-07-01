Skupni podaci o korisniku za tenant. S obzirom na userIds, vraća prikazne informacije iz User / SSOUser.  
Koristi se u widgetu za komentare kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti.  
Nema konteksta stranice: privatnost se provodi jednako (privatni profili su maskirani).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // process response
    }catch(const std::exception&){
        // handle error
    }
});
[inline-code-end]