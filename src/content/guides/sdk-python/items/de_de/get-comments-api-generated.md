## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| skip | integer | query | Nein |  |
| asTree | boolean | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |
| contextUserId | string | query | Nein |  |
| hashTag | string | query | Nein |  |
| parentId | string | query | Nein |  |
| direction | string | query | Nein |  |
| fromDate | integer | query | Nein |  |
| toDate | integer | query | Nein |  |

## Antwort

Gibt zurück: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_comments Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Nachfolgend sind Beispiele für jede Authentifizierungsmethode aufgeführt. Verwenden Sie das Beispiel,
# das Ihren Anwendungsfall bezüglich Authentifizierung erfüllt.

# API-Schlüssel-Authentifizierung konfigurieren: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie unten das Kommentarzeichen, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel festzulegen
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optional)
    limit = 56 # int |  (optional)
    skip = 56 # int |  (optional)
    as_tree = True # bool |  (optional)
    skip_children = 56 # int |  (optional)
    limit_children = 56 # int |  (optional)
    max_tree_depth = 56 # int |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)
    context_user_id = 'context_user_id_example' # str |  (optional)
    hash_tag = 'hash_tag_example' # str |  (optional)
    parent_id = 'parent_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    from_date = 56 # int |  (optional)
    to_date = 56 # int |  (optional)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]