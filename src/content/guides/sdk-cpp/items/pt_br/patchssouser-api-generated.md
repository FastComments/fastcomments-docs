## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Sim |  |
| updateComments | bool | Não |  |

## Resposta

Retorna: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de patchSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData updateData;
updateData.email = utility::string_t(U"user@example.com");
updateData.displayName = utility::string_t(U"Jane Doe");
boost::optional<bool> updateComments = true;
auto responseHolder = std::make_shared<PatchSSOUserAPIResponse>();
api->patchSSOUser(utility::string_t(U"my-tenant-123"), utility::string_t(U"user@example.com"), updateData, updateComments)
.then([responseHolder](std::shared_ptr<PatchSSOUserAPIResponse> resp){
    if (resp) *responseHolder = *resp;
    return responseHolder;
});
[inline-code-end]

---