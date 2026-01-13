## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| editKey | string | query | No |  |

## Risposta

Restituisce: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_comment_vote_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio DeleteVote'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // stringa | 
	id := "id_example" // stringa | 
	editKey := "editKey_example" // stringa |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteVote(context.Background(), id).TenantId(tenantId).EditKey(editKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DeleteVote`: DeleteCommentVote200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteVote`: %v\n", resp)
}
[inline-code-end]

---