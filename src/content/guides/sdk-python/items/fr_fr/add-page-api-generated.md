---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Renvoie : [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_page_api_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple add_page'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_page_api_response import AddPageAPIResponse
from client.models.create_api_page_data import CreateAPIPageData
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer un préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_api_page_data = client.CreateAPIPageData() # CreateAPIPageData | 

    try:
        api_response = api_instance.add_page(tenant_id, create_api_page_data)
        print("The response of DefaultApi->add_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_page: %s\n" % e)
[inline-code-end]

---