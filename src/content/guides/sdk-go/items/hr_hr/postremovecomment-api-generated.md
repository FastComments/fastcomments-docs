## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Response

Vraća: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_post_remove_comment_api_response.go)

## Example

[inline-code-attrs-start title = 'PostRemoveComment Primjer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostRemoveComment(context.Background(), commentId).TenantId(tenantId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Pogreška pri pozivu `ModerationAPI.PostRemoveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Cjelokupni HTTP odgovor: %v\n", r)
	}
	// odgovor od `PostRemoveComment`: PostRemoveCommentApiResponse
	fmt.Fprintf(os.Stdout, "Odgovor od `ModerationAPI.PostRemoveComment`: %v\n", resp)
}
[inline-code-end]

---