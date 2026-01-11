## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| skip | integer | query | Ne |  |
| asTree | boolean | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |
| contextUserId | string | query | Ne |  |
| hashTag | string | query | Ne |  |
| parentId | string | query | Ne |  |
| direction | string | query | Ne |  |

## Odgovor

Vrača: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in avtorizacije
# v skladu z varnostno politiko strežnika API.
# Spodaj so navedeni primeri za vsak način overjanja, uporabite tistega
# ki ustreza vašemu primeru uporabe.
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (izbirno)
    limit = 56 # int |  (izbirno)
    skip = 56 # int |  (izbirno)
    as_tree = True # bool |  (izbirno)
    skip_children = 56 # int |  (izbirno)
    limit_children = 56 # int |  (izbirno)
    max_tree_depth = 56 # int |  (izbirno)
    url_id = 'url_id_example' # str |  (izbirno)
    user_id = 'user_id_example' # str |  (izbirno)
    anon_user_id = 'anon_user_id_example' # str |  (izbirno)
    context_user_id = 'context_user_id_example' # str |  (izbirno)
    hash_tag = 'hash_tag_example' # str |  (izbirno)
    parent_id = 'parent_id_example' # str |  (izbirno)
    direction = client.SortDirections() # SortDirections |  (izbirno)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]

---