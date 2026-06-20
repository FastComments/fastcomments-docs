## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| trustFactor | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_user_trust_factor_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de set_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_user_trust_factor_response import SetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# A definição do host é opcional e, por padrão, é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (opcional)
    trust_factor = 'trust_factor_example' # str |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.set_trust_factor(user_id=user_id, trust_factor=trust_factor, sso=sso)
        print("The response of ModerationApi->set_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->set_trust_factor: %s\n" % e)
[inline-code-end]