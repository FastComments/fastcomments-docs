## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nee |  |
| state | double | Nee |  |
| skip | double | Nee |  |
| limit | double | Nee |  |

## Response

Retourneert: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTicketsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getTickets Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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