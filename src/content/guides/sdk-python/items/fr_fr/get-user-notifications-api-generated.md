## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Non | Permet de déterminer si la page actuelle est abonnée. |
| pageSize | integer | query | Non |  |
| afterId | string | query | Non |  |
| includeContext | boolean | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| includeTranslations | boolean | query | Non |  |
| includeTenantNotifications | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est facultative et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Permet de déterminer si la page actuelle est abonnée. (optionnel)
    page_size = 56 # int |  (optionnel)
    after_id = 'after_id_example' # str |  (optionnel)
    include_context = True # bool |  (optionnel)
    after_created_at = 56 # int |  (optionnel)
    unread_only = True # bool |  (optionnel)
    dm_only = True # bool |  (optionnel)
    no_dm = True # bool |  (optionnel)
    include_translations = True # bool |  (optionnel)
    include_tenant_notifications = True # bool |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]