## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Odgovor

Vraća: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## Primjer

[inline-code-attrs-start title = 'flag_comment Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu sa politikom sigurnosti API servera.
# Primjeri za svaki metod autentifikacije su dati ispod, koristite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (neobavezno)
    anon_user_id = 'anon_user_id_example' # str |  (neobavezno)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]