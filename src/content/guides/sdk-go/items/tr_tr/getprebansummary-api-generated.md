## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Yanıt

Returns: [`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## Örnek

[inline-code-attrs-start title = 'GetPreBanSummary Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool | (opsiyonel)
	includeByIP := true // bool | (opsiyonel)
	includeByEmailDomain := true // bool | (opsiyonel)
	sso := "sso_example" // string | (opsiyonel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).TenantId(tenantId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.GetPreBanSummary` çağrılırken hata: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `GetPreBanSummary`'den yanıt: PreBanSummary
	fmt.Fprintf(os.Stdout, "Yanıt `ModerationAPI.GetPreBanSummary`'den: %v\n", resp)
}
[inline-code-end]