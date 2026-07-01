## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Yanıt

Döndürür: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_from_comment_response.go)

## Örnek

[inline-code-attrs-start title = 'GetBanUsersFromComment Örnek'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetBanUsersFromComment(context.Background(), commentId).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.GetBanUsersFromComment` çağırılırken hata: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `GetBanUsersFromComment`'dan gelen yanıt: GetBannedUsersFromCommentResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.GetBanUsersFromComment`'den yanıt: %v\n", resp)
}
[inline-code-end]