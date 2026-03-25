## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| userId | string | Sì |  |
| id | string | Sì |  |
| changeTicketStateBody | ChangeTicketStateBody | Sì |  |

## Risposta

Restituisce: [`ChangeTicketState_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketState_200_response.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di changeTicketState'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("support@acme.com");
utility::string_t ticketId = U("ticket-456");

ChangeTicketStateBody changeBody;
changeBody.state = U("closed");
changeBody.note = boost::optional<utility::string_t>(U("Resolved after customer confirmation"));

api->changeTicketState(tenantId, userId, ticketId, changeBody)
.then([](std::shared_ptr<ChangeTicketState_200_response> resp){
    auto copy = std::make_shared<ChangeTicketState_200_response>(*resp);
    return copy;
})
.then([](std::shared_ptr<ChangeTicketState_200_response> finalResp){
    (void)finalResp;
});
[inline-code-end]

---