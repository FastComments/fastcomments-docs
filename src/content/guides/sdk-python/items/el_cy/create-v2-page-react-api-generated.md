## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| id | string | query | Ναι |  |
| title | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα create_v2_page_react'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα πλαίσιο με ένα στιγμιότυπο του ApiClient
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 
    title = 'title_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.create_v2_page_react(tenant_id, url_id, id, title=title)
        print("The response of PublicApi->create_v2_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_v2_page_react: %s\n" % e)
[inline-code-end]