## Parametri

| Name | Type | Location | Required | Description |
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
| fromDate | integer | query | Ne |  |
| toDate | integer | query | Ne |  |

## Odgovor

Vraća: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentifikacije navedeni su dolje, koristite onaj
# koji odgovara vašem scenariju korištenja autentifikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst koristeći primjerak API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte primjerak API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (neobavezno)
    limit = 56 # int |  (neobavezno)
    skip = 56 # int |  (neobavezno)
    as_tree = True # bool |  (neobavezno)
    skip_children = 56 # int |  (neobavezno)
    limit_children = 56 # int |  (neobavezno)
    max_tree_depth = 56 # int |  (neobavezno)
    url_id = 'url_id_example' # str |  (neobavezno)
    user_id = 'user_id_example' # str |  (neobavezno)
    anon_user_id = 'anon_user_id_example' # str |  (neobavezno)
    context_user_id = 'context_user_id_example' # str |  (neobavezno)
    hash_tag = 'hash_tag_example' # str |  (neobavezno)
    parent_id = 'parent_id_example' # str |  (neobavezno)
    direction = client.SortDirections() # SortDirections |  (neobavezno)
    from_date = 56 # int |  (neobavezno)
    to_date = 56 # int |  (neobavezno)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]