## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Response

Vrne: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Primer

[inline-code-attrs-start title = 'get_comments Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre avtentikacije in avtorizacije v skladu s politiko varnosti strežnika API.
# Primeri za vsako metodo avtentikacije so predstavljeni spodaj, uporabite primer, ki ustreza vašemu primeru uporabe avtentikacije.

# Nastavi avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvari instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (neobvezno)
    limit = 56 # int |  (neobvezno)
    skip = 56 # int |  (neobvezno)
    as_tree = True # bool |  (neobvezno)
    skip_children = 56 # int |  (neobvezno)
    limit_children = 56 # int |  (neobvezno)
    max_tree_depth = 56 # int |  (neobvezno)
    url_id = 'url_id_example' # str |  (neobvezno)
    user_id = 'user_id_example' # str |  (neobvezno)
    anon_user_id = 'anon_user_id_example' # str |  (neobvezno)
    context_user_id = 'context_user_id_example' # str |  (neobvezno)
    hash_tag = 'hash_tag_example' # str |  (neobvezno)
    parent_id = 'parent_id_example' # str |  (neobvezno)
    direction = client.SortDirections() # SortDirections |  (neobvezno)
    from_date = 56 # int |  (neobvezno)
    to_date = 56 # int |  (neobvezno)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("Odgovor DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Izjema pri klicanju DefaultApi->get_comments: %s\n" % e)
[inline-code-end]