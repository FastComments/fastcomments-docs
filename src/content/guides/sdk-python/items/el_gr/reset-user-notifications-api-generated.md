## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| afterId | string | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/reset_user_notifications_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'reset_user_notifications Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.reset_user_notifications_response import ResetUserNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Χρησιμοποιήστε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία ενός στιγμιότυπου της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    after_id = 'after_id_example' # str |  (προαιρετικό)
    after_created_at = 56 # int |  (προαιρετικό)
    unread_only = True # bool |  (προαιρετικό)
    dm_only = True # bool |  (προαιρετικό)
    no_dm = True # bool |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.reset_user_notifications(tenant_id, after_id=after_id, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, sso=sso)
        print("The response of PublicApi->reset_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->reset_user_notifications: %s\n" % e)
[inline-code-end]

---