## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| yearNumber | number | query | Non |  |
| monthNumber | number | query | Non |  |
| dayNumber | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Retourne: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_daily_usages_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_tenant_daily_usages'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_daily_usages_response import GetTenantDailyUsagesResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous ; utilisez celui
# qui correspond à votre cas d'utilisation d'authentification.

# Configurer l'authentification par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour ajouter un préfixe (par ex. Bearer) à la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    year_number = 3.4 # float |  (facultatif)
    month_number = 3.4 # float |  (facultatif)
    day_number = 3.4 # float |  (facultatif)
    skip = 3.4 # float |  (facultatif)

    try:
        api_response = api_instance.get_tenant_daily_usages(tenant_id, year_number=year_number, month_number=month_number, day_number=day_number, skip=skip)
        print("The response of DefaultApi->get_tenant_daily_usages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_daily_usages: %s\n" % e)
[inline-code-end]