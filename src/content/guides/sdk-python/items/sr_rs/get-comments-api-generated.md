## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | упит | Да |  |
| page | integer | упит | Не |  |
| limit | integer | упит | Не |  |
| skip | integer | упит | Не |  |
| asTree | boolean | упит | Не |  |
| skipChildren | integer | упит | Не |  |
| limitChildren | integer | упит | Не |  |
| maxTreeDepth | integer | упит | Не |  |
| urlId | string | упит | Не |  |
| userId | string | упит | Не |  |
| anonUserId | string | упит | Не |  |
| contextUserId | string | упит | Не |  |
| hashTag | string | упит | Не |  |
| parentId | string | упит | Не |  |
| direction | string | упит | Не |  |
| fromDate | integer | упит | Не |  |
| toDate | integer | упит | Не |  |

## Одговор

Returns: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Пример

[inline-code-attrs-start title = 'get_comments Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent treba da konfiguriše parametre autentikacije i autorizacije
# u skladu sa bezbednosnom politikom API servera.
# Primeri za svaki metod autentikacije su dati ispod, koristite primer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (opciono)
    limit = 56 # int |  (opciono)
    skip = 56 # int |  (opciono)
    as_tree = True # bool |  (opciono)
    skip_children = 56 # int |  (opciono)
    limit_children = 56 # int |  (opciono)
    max_tree_depth = 56 # int |  (opciono)
    url_id = 'url_id_example' # str |  (opciono)
    user_id = 'user_id_example' # str |  (opciono)
    anon_user_id = 'anon_user_id_example' # str |  (opciono)
    context_user_id = 'context_user_id_example' # str |  (opciono)
    hash_tag = 'hash_tag_example' # str |  (opciono)
    parent_id = 'parent_id_example' # str |  (opciono)
    direction = client.SortDirections() # SortDirections |  (opciono)
    from_date = 56 # int |  (opciono)
    to_date = 56 # int |  (opciono)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]