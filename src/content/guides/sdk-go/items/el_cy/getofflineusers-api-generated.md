Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτήν τη στιγμή online. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Μέλη".
Σελιδοποίηση με cursor βάσει commenterName: ο διακομιστής χρησιμοποιεί το μερικό ευρετήριο {tenantId, urlId, commenterName}
ευρετήριο από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρίζεται στην πλευρά του διακομιστή). |
| afterName | string | query | No | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Δεσμευτής ισοβαθμίας cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπουν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Αναγνωριστικό URL σελίδας (καθαρίζεται στην πλευρά του διακομιστή).
	afterName := "afterName_example" // string | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. (optional)
	afterUserId := "afterUserId_example" // string | Δεσμευτής ισοβαθμίας cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπουν εγγραφές. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]