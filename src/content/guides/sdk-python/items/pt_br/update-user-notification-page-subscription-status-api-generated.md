Ativar ou desativar notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas
para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Sim |  |
| url | string | query | Sim |  |
| pageTitle | string | query | Sim |  |
| subscribedOrUnsubscribed | string | path | Sim |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_page_subscription_status_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de update_user_notification_page_subscription_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_page_subscription_status_response import UpdateUserNotificationPageSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    url = 'url_example' # str | 
    page_title = 'page_title_example' # str | 
    subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example' # str | 
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso=sso)
        print("The response of PublicApi->update_user_notification_page_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_page_subscription_status: %s\n" % e)
[inline-code-end]