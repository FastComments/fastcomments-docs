Otpremi i promijeni veličinu slike

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predložak veličine: "Default" (1000x1000px) ili "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | No | ID stranice s koje se vrši prijenos, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Predložak veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) (opcionalno)
    url_id = 'url_id_example' # str | ID stranice s koje se vrši prijenos, za konfiguraciju (opcionalno)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]