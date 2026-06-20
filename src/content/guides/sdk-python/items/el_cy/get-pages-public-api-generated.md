Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να γεμίσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στη λυμένη προσαρμοσμένη διαμόρφωση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του αιτούμενου χρήστη.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής δρομέας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. |
| sortBy | string | query | No | Διάταξη ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Εάν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και ως προεπιλογή χρησιμοποιείται το https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Αδιαφανής δρομέας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`. (προαιρετικό)
    limit = 56 # int | 1..200, προεπιλογή 50 (προαιρετικό)
    q = 'q_example' # str | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. (προαιρετικό)
    sort_by = client.PagesSortBy() # PagesSortBy | Διάταξη ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). (προαιρετικό)
    has_comments = True # bool | Εάν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. (προαιρετικό)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]