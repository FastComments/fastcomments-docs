## Parameters

| Naam | Type | Verplicht | Omschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nee |  |

## Respons

Retourneert: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicketResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getTicket Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ticketId = U("ticket-789");
boost::optional<utility::string_t> userId = U("alice@example.com");
api->getTicket(tenantId, ticketId, userId).then([](pplx::task<std::shared_ptr<GetTicketResponse>> task){
    try{
        auto resp = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]