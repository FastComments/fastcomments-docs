## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| spam | boolean | query | Non |  |
| permNotSpam | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_set_comment_spam_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (optionnel)
    perm_not_spam = True # bool |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.post_set_comment_spam_status(comment_id, spam=spam, perm_not_spam=perm_not_spam, sso=sso)
        print("The response of ModerationApi->post_set_comment_spam_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_spam_status: %s\n" % e)
[inline-code-end]