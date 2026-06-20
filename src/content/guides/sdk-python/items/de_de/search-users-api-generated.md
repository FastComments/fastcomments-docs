## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nein |  |
| mentionGroupIds | array | query | Nein |  |
| sso | string | query | Nein |  |
| searchSection | string | query | Nein |  |

## Antwort

Gibt zurück: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users_result.py)

## Beispiel

[inline-code-attrs-start title = 'search_users Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users_result import SearchUsersResult
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und Standard ist https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (optional)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (optional)
    sso = 'sso_example' # str |  (optional)
    search_section = 'search_section_example' # str |  (optional)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]