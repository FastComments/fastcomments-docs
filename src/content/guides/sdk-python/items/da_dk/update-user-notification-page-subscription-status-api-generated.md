Aktivér eller deaktiver notifikationer for en side. Når brugere er tilmeldt en side, oprettes notifikationer
for nye rodkommentarer, og også

## Parameters

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| url | string | query | Ja |  |
| pageTitle | string | query | Ja |  |
| subscribedOrUnsubscribed | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_page_subscription_status_response.py)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på update_user_notification_page_subscription_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_page_subscription_status_response import UpdateUserNotificationPageSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, og standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Gå ind i en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    url = 'url_example' # str | 
    page_title = 'page_title_example' # str | 
    subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example' # str | 
    sso = 'sso_example' # str |  (valgfri)

    try:
        api_response = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso=sso)
        print("The response of PublicApi->update_user_notification_page_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_page_subscription_status: %s\n" % e)
[inline-code-end]