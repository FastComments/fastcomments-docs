## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| redirectURL | string | Nee |  |

## Response

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Example

[inline-code-attrs-start title = 'sendLoginLink Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->sendLoginLink(
    U("my-tenant-123"),
    U("user@example.com"),
    boost::make_optional(U("https://myapp.com/auth/callback"))
).then([](std::shared_ptr<APIEmptyResponse> resp) {
});
[inline-code-end]