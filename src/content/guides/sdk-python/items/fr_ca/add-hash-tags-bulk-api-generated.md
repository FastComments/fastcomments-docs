## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Non |  |

## Réponse

Renvoie : [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple add_hash_tags_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Consultez configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous, utilisez l'exemple qui
# répond à votre cas d'utilisation d'authentification.

# Configurez l'autorisation par clé API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour définir un préfixe (p. ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (optionnel)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (optionnel)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]