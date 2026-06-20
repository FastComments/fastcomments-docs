## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| direction | string | query | Non |  |
| repliesToUserId | string | query | Non |  |
| page | number | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| isCrawler | boolean | query | Non |  |

## Réponse

Retourne : [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut c'est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ouvrez un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (optionnel)
    direction = client.SortDirections() # SortDirections |  (optionnel)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (optionnel)
    page = 3.4 # float |  (optionnel)
    includei10n = True # bool |  (optionnel)
    locale = 'locale_example' # str |  (optionnel)
    is_crawler = True # bool |  (optionnel)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]