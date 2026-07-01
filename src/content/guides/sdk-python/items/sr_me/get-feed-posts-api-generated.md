req
tenantId
afterId

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Response

Returns: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Example

[inline-code-attrs-start title = 'get_feed_posts Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetFeedPostsOptions
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumevano na https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije u skladu sa sigurnosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji odgovara vašem slučaju upotrebe.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (optional)
    limit = 56 # int |  (optional)
    tags = ['tags_example'] # List[str] |  (optional)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, GetFeedPostsOptions(after_id=after_id, limit=limit, tags=tags))
        print("Odgovor DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom poziva DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]