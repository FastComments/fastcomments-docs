## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Risposta

Restituisce: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateHashTagResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di addHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
CreateHashTagBody createBody;
createBody.name = utility::string_t(U("release"));
createBody.createdBy = utility::string_t(U("admin@example.com"));
auto bodyOpt = boost::optional<CreateHashTagBody>(createBody);

api->addHashTag(tenantId, bodyOpt).then([](pplx::task<std::shared_ptr<CreateHashTagResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "HashTag created successfully\n";
        } else {
            auto fallback = std::make_shared<CreateHashTagResponse>();
        }
    } catch (const std::exception &e) {
        std::cerr << "AddHashTag failed: " << e.what() << '\n';
    }
});
[inline-code-end]

---