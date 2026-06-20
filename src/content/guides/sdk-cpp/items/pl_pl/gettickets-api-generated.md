## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| state | double | Nie |  |
| skip | double | Nie |  |
| limit | double | Nie |  |

## Odpowiedź

Zwraca: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicketsResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getTickets'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId{ utility::string_t(U("user@example.com")) };
boost::optional<double> state{ 1.0 };
boost::optional<double> skip{ 0.0 };
boost::optional<double> limit{ 25.0 };
auto emptyResp = std::make_shared<GetTicketsResponse>();
api->getTickets(tenantId, userId, state, skip, limit)
.then([](pplx::task<std::shared_ptr<GetTicketsResponse>> t){
    try {
        auto resp = t.get();
        if (resp) (void)resp;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---