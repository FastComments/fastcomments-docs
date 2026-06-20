## Parametre

| Name | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| skip | integer | query | Nej |  |
| asTree | boolean | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |
| contextUserId | string | query | Nej |  |
| hashTag | string | query | Nej |  |
| parentId | string | query | Nej |  |
| direction | string | query | Nej |  |
| fromDate | integer | query | Nej |  |
| toDate | integer | query | Nej |  |

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_comments Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Angivelse af host er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klienten skal konfigurere autentificering og autorisationsparametre
# i overensstemmelse med API-serverens sikkerhedspolitik.
# Eksempler for hver auth-metode er vist nedenfor; brug eksemplet der
# passer til din auth-brugssituation.

# Konfigurer API-nøgleautorisering: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Fjern kommentaren nedenfor for at sætte et prefix (f.eks. Bearer) for API-nøglen, hvis nødvendigt
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (valgfri)
    limit = 56 # int |  (valgfri)
    skip = 56 # int |  (valgfri)
    as_tree = True # bool |  (valgfri)
    skip_children = 56 # int |  (valgfri)
    limit_children = 56 # int |  (valgfri)
    max_tree_depth = 56 # int |  (valgfri)
    url_id = 'url_id_example' # str |  (valgfri)
    user_id = 'user_id_example' # str |  (valgfri)
    anon_user_id = 'anon_user_id_example' # str |  (valgfri)
    context_user_id = 'context_user_id_example' # str |  (valgfri)
    hash_tag = 'hash_tag_example' # str |  (valgfri)
    parent_id = 'parent_id_example' # str |  (valgfri)
    direction = client.SortDirections() # SortDirections |  (valgfri)
    from_date = 56 # int |  (valgfri)
    to_date = 56 # int |  (valgfri)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]