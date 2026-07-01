Bulk brugerinfo for en lejer. Givet userIds returneres visningsinfo fra User / SSOUser.  
Brugt af kommentarfunktionen til at berige brugere, der netop er dukket op via en tilstedeværelseshændelse.  
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // behandl svar
    }catch(const std::exception&){
        // håndtér fejl
    }
});
[inline-code-end]