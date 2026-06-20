## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_manual_badges_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_manual_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_manual_badges_response import GetTenantManualBadgesResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut : https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_manual_badges(sso=sso)
        print("The response of ModerationApi->get_manual_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_manual_badges: %s\n" % e)
[inline-code-end]