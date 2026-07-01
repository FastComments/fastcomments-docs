## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| badgeId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| commentId | string | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_remove_user_badge_response.go)

## Örnek

[inline-code-attrs-start title = 'PutRemoveBadge Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	badgeId := "badgeId_example" // string | 
	userId := "userId_example" // string |  (opsiyonel)
	commentId := "commentId_example" // string |  (opsiyonel)
	broadcastId := "broadcastId_example" // string |  (opsiyonel)
	sso := "sso_example" // string |  (opsiyonel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PutRemoveBadge(context.Background()).TenantId(tenantId).BadgeId(badgeId).UserId(userId).CommentId(commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.PutRemoveBadge`` çağrılırken hata: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `PutRemoveBadge`'den yanıt: RemoveUserBadgeResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.PutRemoveBadge`'den Yanıt: %v\n", resp)
}
[inline-code-end]

---