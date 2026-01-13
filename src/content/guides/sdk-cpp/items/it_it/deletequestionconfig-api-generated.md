## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteQuestionConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("qcfg-98765");
boost::optional<utility::string_t> tenantOpt = tenantId;
auto t = api->deleteQuestionConfig(tenantOpt.value_or(utility::string_t()), id)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp) -> std::shared_ptr<FlagCommentPublic_200_response> {
    auto out = std::make_shared<FlagCommentPublic_200_response>();
    if (resp) *out = *resp;
    return out;
});
t.wait();
[inline-code-end]

---