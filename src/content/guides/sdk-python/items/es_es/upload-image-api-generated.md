Subir y redimensionar una imagen

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| sizePreset | string | query | No | Preajuste de tamaño: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | No | ID de página desde la cual se realiza la subida, para configurar |

## Respuesta

Devuelve: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para obtener una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea tamaños para dispositivos populares) (opcional)
    url_id = 'url_id_example' # str | ID de página desde la cual se realiza la subida, para configurar (opcional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]