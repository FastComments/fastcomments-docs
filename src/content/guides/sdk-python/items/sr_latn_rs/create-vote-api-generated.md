## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Da |  |
| direction | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Primer

[inline-code-attrs-start title = 'create_vote Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i po defaultu je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Napravite instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]