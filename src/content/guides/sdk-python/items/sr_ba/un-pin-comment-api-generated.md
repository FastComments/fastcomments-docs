## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`PinComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pin_comment200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer un_pin_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pin_comment200_response import PinComment200Response
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je neobavezno i podrazumijeva se https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.un_pin_comment(tenant_id, comment_id, broadcast_id, sso=sso)
        print("The response of PublicApi->un_pin_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->un_pin_comment: %s\n" % e)
[inline-code-end]