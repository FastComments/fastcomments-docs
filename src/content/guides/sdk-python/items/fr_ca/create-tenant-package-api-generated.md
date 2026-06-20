## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |

## Réponse

Retourne: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_tenant_package'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.models.create_tenant_package_response import CreateTenantPackageResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et la valeur par défaut est https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'auth sont fournis ci‑dessous ; utilisez celui
# qui correspond à votre cas d'utilisation d'authentification.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]