Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να γεμίσει τη λίστα δωματίων του.
Απαιτεί να είναι true το `enableFChat` στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδων του αιτούμενου χρήστη.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συσχετίζεται με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου ανεξαρτήτως πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Εάν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	cursor := "cursor_example" // string | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συσχετίζεται με το ίδιο `sortBy`. (προαιρετικό)
	limit := int32(56) // int32 | 1..200, προεπιλογή 50 (προαιρετικό)
	q := "q_example" // string | Προαιρετικό φίλτρο προθέματος τίτλου ανεξαρτήτως πεζών-κεφαλαίων. (προαιρετικό)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά). (προαιρετικό)
	hasComments := true // bool | Εάν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]