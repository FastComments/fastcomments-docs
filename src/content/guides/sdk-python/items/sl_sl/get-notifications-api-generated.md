## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| userId | string | poizvedba | Ne |  |
| urlId | string | poizvedba | Ne |  |
| fromCommentId | string | poizvedba | Ne |  |
| viewed | boolean | poizvedba | Ne |  |
| type | string | poizvedba | Ne |  |
| skip | number | poizvedba | Ne |  |

## Odgovor

Vrne: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications_response.py)

## Primer

[inline-code-attrs-start title = 'get_notifications Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationsOptions
from client.models.get_notifications_response import GetNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si datoteko configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre za avtentikacijo in avtorizacijo
# v skladu s politiko varnosti strežnika API.
# Primeri za vsako metodo avtentikacije so navedeni spodaj, uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Nastavite avtorizacijo API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    url_id = 'url_id_example' # str |  (neobvezno)
    from_comment_id = 'from_comment_id_example' # str |  (neobvezno)
    viewed = True # bool |  (neobvezno)
    type = 'type_example' # str |  (neobvezno)
    skip = 3.4 # float |  (neobvezno)

    try:
        api_response = api_instance.get_notifications(tenant_id, GetNotificationsOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip))
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]