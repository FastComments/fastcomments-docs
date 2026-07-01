## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_ban_status_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetCommentBanStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCommentBanStatus(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Greška prilikom poziva `ModerationAPI.GetCommentBanStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Cijeli HTTP odgovor: %v\n", r)
	}
	// odgovor iz `GetCommentBanStatus`: GetCommentBanStatusResponse
	fmt.Fprintf(os.Stdout, "Odgovor iz `ModerationAPI.GetCommentBanStatus`: %v\n", resp)
}
[inline-code-end]

---