## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user200_response import GetVotesForUser200Response
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati u nastavku, koristite primer koji
# zadovoljava vaš slučaj upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (opciono)
    anon_user_id = 'anon_user_id_example' # str |  (opciono)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]

---