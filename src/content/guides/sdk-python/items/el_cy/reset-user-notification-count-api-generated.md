## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα reset_user_notification_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications200_response import ResetUserNotifications200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ανοίξτε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.reset_user_notification_count(tenant_id, sso=sso)
        print("The response of PublicApi->reset_user_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notification_count: %s\n" % e)
[inline-code-end]