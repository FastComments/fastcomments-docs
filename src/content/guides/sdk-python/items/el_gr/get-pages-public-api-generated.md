Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop client του FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτεί το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του αιτούμενου χρήστη.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συσχετίζεται με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου, χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτα τα πιο πρόσφατα), `commentCount` (πρώτα αυτά με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν είναι true, επέστρεψε μόνο σελίδες που έχουν τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συσχετίζεται με το ίδιο `sortBy`. (optional)
    limit = 56 # int | 1..200, προεπιλογή 50 (optional)
    q = 'q_example' # str | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, πρώτα τα πιο πρόσφατα), `commentCount` (πρώτα αυτά με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). (optional)
    has_comments = True # bool | Αν είναι true, επιστρέψτε μόνο σελίδες με τουλάχιστον ένα σχόλιο. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]