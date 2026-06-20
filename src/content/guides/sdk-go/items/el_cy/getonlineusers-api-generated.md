Αυτή τη στιγμή online θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή τώρα.
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | Όχι | tiebreaker του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπονται. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
	afterName := "afterName_example" // string | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. (προαιρετικό)
	afterUserId := "afterUserId_example" // string | tiebreaker του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπονται. (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]