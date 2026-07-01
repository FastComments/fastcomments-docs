List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

Λίστα σελίδων για έναν ενοικιαστή. Χρησιμοποιείται από το desktop client του FChat για την εμφάνιση της λίστας δωματίων.
Απαιτεί `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη διαμόρφωση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης ομάδας του χρήστη που κάνει το αίτημα.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πλήθος σχολίων πρώτα), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και η προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας παρουσίασης της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`. (προαιρετικό)
    limit = 56 # int | 1..200, προεπιλογή 50 (προαιρετικό)
    q = 'q_example' # str | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. (προαιρετικό)
    sort_by = client.PagesSortBy() # PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πλήθος σχολίων πρώτα), ή `title` (αλφαβητικά). (προαιρετικό)
    has_comments = True # bool | Αν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. (προαιρετικό)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---