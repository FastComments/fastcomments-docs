req
tenantId
afterId

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Odgovor

Vrne: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_feed_posts_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_feed_posts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetFeedPostsOptions
from client.models.get_feed_posts_response import GetFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglej si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
# Odjemalec mora nastaviti parametre avtentikacije in avtorizacije v skladu s varnostno politiko strežnika API.
# Primeri za vsako metodo avtentikacije so spodaj, uporabi primer, ki ustreza tvojemu primeru uporabe avtentikacije.

# Nastavi avtorizacijo API ključa: api_key
# Odkomentiraj spodaj, če želiš nastaviti predpono (npr. Bearer) za API ključ, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopi v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari primerek API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (neobvezno)
    limit = 56 # int |  (neobvezno)
    tags = ['tags_example'] # List[str] |  (neobvezno)

    try:
        api_response = api_instance.get_feed_posts(tenant_id, GetFeedPostsOptions(after_id=after_id, limit=limit, tags=tags))
        print("The response of DefaultApi->get_feed_posts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_feed_posts: %s\n" % e)
[inline-code-end]

---