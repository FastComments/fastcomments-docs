---
Μαζικές πληροφορίες χρηστών για έναν tenant. Δεδομένων userIds, επιστρέφει πληροφορίες προβολής από User / SSOUser.
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει χρήστες που μόλις εμφανίστηκαν μέσω ενός συμβάντος παρουσίας.
Χωρίς πλαίσιο σελίδας: η προστασία προσωπικών δεδομένων εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds διαχωρισμένα με κόμμα. |

## Απόκριση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetUsersInfo'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | userIds διαχωρισμένα με κόμμα.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetUsersInfo`: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]

---