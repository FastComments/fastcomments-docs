## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlIdWS | string | query | Oui |  |
| userIds | string | query | Oui |  |

## Retourne

Retourne: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_presence_statuses200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_presence_statuses'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_presence_statuses200_response import GetUserPresenceStatuses200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut vaut https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id_ws = 'url_id_ws_example' # str | 
    user_ids = 'user_ids_example' # str | 

    try:
        api_response = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
        print("The response of PublicApi->get_user_presence_statuses:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_presence_statuses: %s\n" % e)
[inline-code-end]

---