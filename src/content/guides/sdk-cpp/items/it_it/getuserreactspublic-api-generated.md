## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| options | const GetUserReactsPublicOptions& | Sì |  |

## Risposta

Restituisce: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## Esempio

[inline-code-attrs-start title = 'getUserReactsPublic Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetUserReactsPublicOptions{};
options.userId = boost::optional<utility::string_t>(U("user@example.com"));
options.limit = boost::optional<int>(50);
api->getUserReactsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<UserReactsResponse>> t){
    auto response = t.get();
});
[inline-code-end]

---