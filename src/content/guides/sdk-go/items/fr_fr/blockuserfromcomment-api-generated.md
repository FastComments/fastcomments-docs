## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Réponse

Renvoie : [`BlockSuccess`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_block_success.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de BlockUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	blockFromCommentParams := *openapiclient.NewBlockFromCommentParams() // BlockFromCommentParams | 
	userId := "userId_example" // string |  (facultatif)
	anonUserId := "anonUserId_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.BlockUserFromComment(context.Background(), id).TenantId(tenantId).BlockFromCommentParams(blockFromCommentParams).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.BlockUserFromComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `BlockUserFromComment` : BlockSuccess
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.BlockUserFromComment`: %v\n", resp)
}
[inline-code-end]