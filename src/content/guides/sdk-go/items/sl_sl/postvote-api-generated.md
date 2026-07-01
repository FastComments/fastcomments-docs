## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odziv

Vračanje: [`VoteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_response.go)

## Primer

[inline-code-attrs-start title = 'Primer PostVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	direction := "direction_example" // string |  (neobvezno)
	broadcastId := "broadcastId_example" // string |  (neobvezno)
	sso := "sso_example" // string |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostVote(context.Background(), commentId).TenantId(tenantId).Direction(direction).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Napaka pri klicu `ModerationAPI.PostVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Poln HTTP odziv: %v\n", r)
	}
	// odgovor iz `PostVote`: VoteResponse
	fmt.Fprintf(os.Stdout, "Odziv iz `ModerationAPI.PostVote`: %v\n", resp)
}
[inline-code-end]