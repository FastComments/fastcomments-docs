Ενεργοποίηση ή απενεργοποίηση ειδοποιήσεων για ένα συγκεκριμένο σχόλιο.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| notificationId | string | path | Ναι |  |
| optedInOrOut | string | path | Ναι |  |
| commentId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_comment_subscription_status_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_comment_subscription_status_response import UpdateUserNotificationCommentSubscriptionStatusResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και από προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία ενός instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    opted_in_or_out = 'opted_in_or_out_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso=sso)
        print("The response of PublicApi->update_user_notification_comment_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_comment_subscription_status: %s\n" % e)
[inline-code-end]