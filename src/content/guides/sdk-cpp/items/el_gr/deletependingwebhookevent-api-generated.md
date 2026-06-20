---
## Παράμετροι

| Όνομα | Τύπος | Απαραίτητο | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deletePendingWebhookEvent'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t eventId = U("wh_ev_987654321");
boost::optional<utility::string_t> requestId = U("req-20260619-01");
api->deletePendingWebhookEvent(tenantId, eventId).then([requestId](std::shared_ptr<APIEmptyResponse> resp) {
    if (resp) {
        auto ack = std::make_shared<APIEmptyResponse>(*resp);
    }
});
[inline-code-end]

---