Omogućite ili onemogućite obaveštenja za određeni komentar.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| notificationId | string | path | Da |  |
| optedInOrOut | string | path | Da |  |
| commentId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_comment_subscription_status_response.py)

## Primer

[inline-code-attrs-start title = 'Primer update_user_notification_comment_subscription_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_comment_subscription_status_response import UpdateUserNotificationCommentSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    opted_in_or_out = 'opted_in_or_out_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso=sso)
        print("The response of PublicApi->update_user_notification_comment_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_comment_subscription_status: %s\n" % e)
[inline-code-end]