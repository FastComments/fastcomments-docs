Bulk‑Benutzerinformationen für einen Mandanten. Angesichts von userIds wird Anzeigedaten von User / SSOUser zurückgegeben.  
Wird vom Kommentar‑Widget verwendet, um Benutzer zu erweitern, die gerade über ein Präsenzereignis erschienen sind.  
Kein Seitenkontext: Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'Beispiel getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // Antwort verarbeiten
    }catch(const std::exception&){
        // Fehler behandeln
    }
});
[inline-code-end]