## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| namespace | string | path | Sim |  |
| component | string | path | Sim |  |
| locale | string | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |

## Resposta

Retorna: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_translations_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_translations'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetTranslationsOptions
from client.models.get_translations_response import GetTranslationsResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Consulte configuration.py para a lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
    api_instance = client.PublicApi(api_client)
    namespace = 'namespace_example' # str | 
    component = 'component_example' # str | 
    locale = 'locale_example' # str |  (opcional)
    use_full_translation_ids = True # bool |  (opcional)

    try:
        api_response = api_instance.get_translations(namespace, component, GetTranslationsOptions(locale=locale, use_full_translation_ids=use_full_translation_ids))
        print("A resposta de PublicApi->get_translations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exceção ao chamar PublicApi->get_translations: %s\n" % e)
[inline-code-end]