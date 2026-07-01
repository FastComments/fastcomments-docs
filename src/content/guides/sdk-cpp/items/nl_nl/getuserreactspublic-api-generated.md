---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| options | const GetUserReactsPublicOptions& | Ja |  |

## Reactie

Retourneert: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getUserReactsPublic Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetUserReactsPublicOptions{};
options.userId = boost::optional<utility::string_t>(U("user@example.com"));
options.limit = boost::optional<int>(50);
api->getUserReactsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<UserReactsResponse>> t){
    auto response = t.get();
});
[inline-code-end]

---