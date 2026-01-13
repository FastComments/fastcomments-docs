## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user200_response import GetVotesForUser200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaki način autentikacije navedeni su dolje, koristite primjer koji
# odgovara vašem slučaju upotrebe.

# Konfigurirajte autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ako je potrebno, otkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]

---