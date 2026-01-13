Carregar e redimensionar uma imagem

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| sizePreset | string | query | Não | Predefinição de tamanho: "Default" (1000x1000px) ou "CrossPlatform" (cria tamanhos para dispositivos populares) |
| urlId | string | query | Não | ID da página de onde o upload está sendo feito, para configurar |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e o padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe da API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) (opcional)
    url_id = 'url_id_example' # str | ID da página de onde o upload está sendo feito, para configurar (opcional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]