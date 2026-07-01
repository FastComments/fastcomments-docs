## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | path | Sim |  |
| banEmail | boolean | query | Não |  |
| banEmailDomain | boolean | query | Não |  |
| banIP | boolean | query | Não |  |
| deleteAllUsersComments | boolean | query | Não |  |
| bannedUntil | string | query | Não |  |
| isShadowBan | boolean | query | Não |  |
| updateId | string | query | Não |  |
| banReason | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostBanUserFromCommentOptions
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (opcional)
    ban_email_domain = True # bool |  (opcional)
    ban_ip = True # bool |  (opcional)
    delete_all_users_comments = True # bool |  (opcional)
    banned_until = 'banned_until_example' # str |  (opcional)
    is_shadow_ban = True # bool |  (opcional)
    update_id = 'update_id_example' # str |  (opcional)
    ban_reason = 'ban_reason_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_ban_user_from_comment(tenant_id, comment_id, PostBanUserFromCommentOptions(ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso))
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]