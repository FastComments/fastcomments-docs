Upload and resize an image
============================

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Podrazumevani set veličina: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice sa koje se vrši otpremanje, za konfiguraciju |

## Response

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Example

[inline-code-attrs-start title = 'upload_image Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Set veličina: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje) (optional)
    url_id = 'url_id_example' # str | ID stranice sa koje se vrši otpremanje, za konfiguraciju (optional)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("Odgovor PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom poziva PublicApi->upload_image: %s\n" % e)
[inline-code-end]

---