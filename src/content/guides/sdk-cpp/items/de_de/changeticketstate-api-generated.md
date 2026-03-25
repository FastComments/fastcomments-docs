## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |
| id | string | Ja |  |
| changeTicketStateBody | ChangeTicketStateBody | Ja |  |

## Antwort

Gibt zurück: [`ChangeTicketState_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ChangeTicketState_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'changeTicketState Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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