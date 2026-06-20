## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| id | string | Sì |  |
| title | string | No |  |

## Risposta

Restituisce: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di createV2PageReact'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("homepage-landing");
utility::string_t id = utility::conversions::to_string_t("user-987@example.com");
boost::optional<utility::string_t> title = boost::optional<utility::string_t>(utility::conversions::to_string_t("Introducing FastComments"));

api->createV2PageReact(tenantId, urlId, id, title)
    .then([](std::shared_ptr<CreateV1PageReact> resp){
        auto createdCopy = std::make_shared<CreateV1PageReact>(*resp);
        return createdCopy;
    })
    .then([](std::shared_ptr<CreateV1PageReact> created){
        (void)created;
    })
    .wait();
[inline-code-end]

---