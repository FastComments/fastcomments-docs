## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| update_apisso_user_data | models::UpdateApissoUserData | Sim |  |
| update_comments | bool | Não |  |

## Resposta

Retorna: [`PatchSsoUserApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_sso_user_api_response.rs)