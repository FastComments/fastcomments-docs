## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| reviewed | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di PostSetCommentReviewStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	reviewed := true // bool |  (opzionale)
	sso := "sso_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentReviewStatus(context.Background(), commentId).Reviewed(reviewed).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentReviewStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `PostSetCommentReviewStatus`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentReviewStatus`: %v\n", resp)
}
[inline-code-end]