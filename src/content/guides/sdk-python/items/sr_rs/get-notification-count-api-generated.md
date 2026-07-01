## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| urlId | string | query | Ne |  |
| fromCommentId | string | query | Ne |  |
| viewed | boolean | query | Ne |  |
| type | string | query | Ne |  |

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

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primera za svaki metod autentikacije su dati ispod, koristite primer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (opciono)
    url_id = 'url_id_example' # str |  (opciono)
    from_comment_id = 'from_comment_id_example' # str |  (opciono)
    viewed = True # bool |  (opciono)
    type = 'type_example' # str |  (opciono)

    try:
        api_response = api_instance.get_notification_count(tenant_id, GetNotificationCountOptions(user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type))
        print("The response of DefaultApi->get_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notification_count: %s\n" % e)
[inline-code-end]