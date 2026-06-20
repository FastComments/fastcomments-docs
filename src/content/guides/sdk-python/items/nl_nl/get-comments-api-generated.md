## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| skip | integer | query | Nee |  |
| asTree | boolean | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| urlId | string | query | Nee |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |
| contextUserId | string | query | Nee |  |
| hashTag | string | query | Nee |  |
| parentId | string | query | Nee |  |
| direction | string | query | Nee |  |
| fromDate | integer | query | Nee |  |
| toDate | integer | query | Nee |  |

## Response

Retourneert: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Het instellen van host is optioneel en standaard ingesteld op https://fastcomments.com
# Zie configuration.py voor een lijst met alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# De client moet de authenticatie- en autorisatieparameters configureren
# in overeenstemming met het beveiligingsbeleid van de API-server.
# Voor elk authenticatiemethode wordt hieronder een voorbeeld gegeven; gebruik het voorbeeld dat
# voldoet aan uw authenticatiegeval.

# Configureer API-sleutelautorisatie: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Haal de commentaarstreep hieronder weg om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Ga een context in met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optioneel)
    limit = 56 # int |  (optioneel)
    skip = 56 # int |  (optioneel)
    as_tree = True # bool |  (optioneel)
    skip_children = 56 # int |  (optioneel)
    limit_children = 56 # int |  (optioneel)
    max_tree_depth = 56 # int |  (optioneel)
    url_id = 'url_id_example' # str |  (optioneel)
    user_id = 'user_id_example' # str |  (optioneel)
    anon_user_id = 'anon_user_id_example' # str |  (optioneel)
    context_user_id = 'context_user_id_example' # str |  (optioneel)
    hash_tag = 'hash_tag_example' # str |  (optioneel)
    parent_id = 'parent_id_example' # str |  (optioneel)
    direction = client.SortDirections() # SortDirections |  (optioneel)
    from_date = 56 # int |  (optioneel)
    to_date = 56 # int |  (optioneel)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]