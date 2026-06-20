## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`PreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pre_ban_summary.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pre_ban_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pre_ban_summary import PreBanSummary
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_by_user_id_and_email = True # bool |  (optionnel)
    include_by_ip = True # bool |  (optionnel)
    include_by_email_domain = True # bool |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_pre_ban_summary(comment_id, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->get_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_pre_ban_summary: %s\n" % e)
[inline-code-end]