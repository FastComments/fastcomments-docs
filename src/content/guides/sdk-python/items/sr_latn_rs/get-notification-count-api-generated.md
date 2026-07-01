## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| urlId | string | query | No |  |
| fromCommentId | string | query | No |  |
| viewed | boolean | query | No |  |
| type | string | query | No |  |

## Odgovor

Vraća: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notification_count_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetNotificationCountOptions
from client.models.get_notification_count_response import GetNotificationCountResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za spisak svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# Primeri za svaki metod autentikacije su dati ispod, koristite primer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    from_comment_id = 'from_comment_id_example' # str |  (optional)
    viewed = True # bool |  (optional)
    type = 'type_example' # str |  (optional)

    try:
        api_response = api_instance.get_notification_count(tenant_id, GetNotificationCountOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type))
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]