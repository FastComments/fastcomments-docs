## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Όχι | Used to determine whether the current page is subscribed. |
| pageSize | integer | query | Όχι |  |
| afterId | string | query | Όχι |  |
| includeContext | boolean | query | Όχι |  |
| afterCreatedAt | integer | query | Όχι |  |
| unreadOnly | boolean | query | Όχι |  |
| dmOnly | boolean | query | Όχι |  |
| noDm | boolean | query | Όχι |  |
| includeTranslations | boolean | query | Όχι |  |
| includeTenantNotifications | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_my_notifications_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_notifications'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_my_notifications_response import GetMyNotificationsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Χρησιμοποιείται για να καθορίσει εάν η τρέχουσα σελίδα είναι εγγεγραμμένη. (προαιρετικό)
    page_size = 56 # int |  (προαιρετικό)
    after_id = 'after_id_example' # str |  (προαιρετικό)
    include_context = True # bool |  (προαιρετικό)
    after_created_at = 56 # int |  (προαιρετικό)
    unread_only = True # bool |  (προαιρετικό)
    dm_only = True # bool |  (προαιρετικό)
    no_dm = True # bool |  (προαιρετικό)
    include_translations = True # bool |  (προαιρετικό)
    include_tenant_notifications = True # bool |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_user_notifications(tenant_id, url_id=url_id, page_size=page_size, after_id=after_id, include_context=include_context, after_created_at=after_created_at, unread_only=unread_only, dm_only=dm_only, no_dm=no_dm, include_translations=include_translations, include_tenant_notifications=include_tenant_notifications, sso=sso)
        print("The response of PublicApi->get_user_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_notifications: %s\n" % e)
[inline-code-end]