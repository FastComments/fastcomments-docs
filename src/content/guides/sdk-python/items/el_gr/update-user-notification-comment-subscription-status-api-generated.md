Ενεργοποίηση ή απενεργοποίηση ειδοποιήσεων για ένα συγκεκριμένο σχόλιο.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| notificationId | string | path | Ναι |  |
| optedInOrOut | string | path | Ναι |  |
| commentId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_notification_status200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα update_user_notification_comment_subscription_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_notification_status200_response import UpdateUserNotificationStatus200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλογή https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    notification_id = 'notification_id_example' # str | 
    opted_in_or_out = 'opted_in_or_out_example' # str | 
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso=sso)
        print("The response of PublicApi->update_user_notification_comment_subscription_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->update_user_notification_comment_subscription_status: %s\n" % e)
[inline-code-end]