Commentateurs précédents sur la page qui NE sont PAS actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Membres".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
index depuis afterName en avant via $gt, sans coût $skip.

## Parameters

| Nom | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Désenchevêtreur de curseur : transmettez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. |

## Response

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Example

[inline-code-attrs-start title = 'Exemple get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifiant d'URL de la page (nettoyé côté serveur).
    after_name = 'after_name_example' # str | Curseur : transmettez nextAfterName de la réponse précédente. (optionnel)
    after_user_id = 'after_user_id_example' # str | Désenchevêtreur de curseur : transmettez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. (optionnel)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]