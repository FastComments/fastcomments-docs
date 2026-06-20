## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Odgovor

Vrne: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Nastavitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre preverjanja pristnosti in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so prikazani primeri za vsako metodo avtentikacije; uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Konfigurirajte avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (neobvezno)
    url_id = 'url_id_example' # str |  (neobvezno)
    from_comment_id = 'from_comment_id_example' # str |  (neobvezno)
    viewed = True # bool |  (neobvezno)
    type = 'type_example' # str |  (neobvezno)

    try:
        api_response = api_instance.get_notification_count(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type)
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]