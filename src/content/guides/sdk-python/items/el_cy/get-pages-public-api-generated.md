List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Απαιτείται το `enableFChat` να είναι αληθές στη λύση του προσαρμοσμένου config για κάθε σελίδα.  
Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης της ομάδας του χρήστη που κάνει το αίτημα.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής κέρσορ σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φιλτράρισμα τίτλου κατά πρόθεμα, χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτες οι πολλές σχολιάσεις), ή `title` (αλφαβητική). |
| hasComments | boolean | query | No | Εάν είναι αληθές, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Η ορισμός του host είναι προαιρετική και προεπιλεγμένη στο https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους παραμετροποίησης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισαγωγή σε ένα context με μια παρουσία του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργία μιας παρουσίας της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Αδιαφανής κέρσορ σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. (optional)
    limit = 56 # int | 1..200, προεπιλογή 50 (optional)
    q = 'q_example' # str | Προαιρετικό φιλτράρισμα τίτλου κατά πρόθεμα, χωρίς διάκριση πεζών-κεφαλαίων. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτες οι πολλές σχολιάσεις), ή `title` (αλφαβητική). (optional)
    has_comments = True # bool | Εάν είναι αληθές, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]