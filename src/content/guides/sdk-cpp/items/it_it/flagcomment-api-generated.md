## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| options | const FlagCommentOptions& | Sì |  |

## Risposta

Restituisce: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di flagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<FlagCommentOptions>();
opts->reason = utility::conversions::to_string_t("spam");
opts->note = boost::optional<utility::string_t>(utility::conversions::to_string_t("User posted duplicate links"));

api->flagComment(utility::conversions::to_string_t("my-tenant-123"),
                 utility::conversions::to_string_t("comment-456"),
                 *opts)
    .then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
        auto resp = t.get();
    });
[inline-code-end]