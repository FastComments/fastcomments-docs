## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_trust_factor_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_trust_factor'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_trust_factor_response import GetUserTrustFactorResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut c'est https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    user_id = 'user_id_example' # str |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_trust_factor(user_id=user_id, sso=sso)
        print("The response of ModerationApi->get_trust_factor:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_trust_factor: %s\n" % e)
[inline-code-end]

---