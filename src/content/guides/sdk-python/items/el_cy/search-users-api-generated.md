## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| usernameStartsWith | string | query | Όχι |  |
| mentionGroupIds | array | query | Όχι |  |
| sso | string | query | Όχι |  |
| searchSection | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'search_users Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία ενός στιγμιότυπου της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (προαιρετικό)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)
    search_section = 'search_section_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]