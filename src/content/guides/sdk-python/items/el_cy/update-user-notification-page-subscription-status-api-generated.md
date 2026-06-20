Ενεργοποιήστε ή απενεργοποιήστε τις ειδοποιήσεις για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις για νέα σχόλια πρώτου επιπέδου, και επίσης

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |
| url | string | query | Ναι |  |
| pageTitle | string | query | Ναι |  |
| subscribedOrUnsubscribed | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_page_subscription_status_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_user_notification_page_subscription_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_page_subscription_status_response import UpdateUserNotificationPageSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένο το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    url = 'url_example' # str | 
    page_title = 'page_title_example' # str | 
    subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example' # str | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso=sso)
        print("The response of PublicApi->update_user_notification_page_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_page_subscription_status: %s\n" % e)
[inline-code-end]