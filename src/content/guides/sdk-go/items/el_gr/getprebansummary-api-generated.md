## Παραμέτρων

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Απάντηση

Επιστρέφει: [`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## Παράδειγμα

[inline-code-attrs-start title = 'GetPreBanSummary Παράδειγμα'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool | (προαιρετικό)
	includeByIP := true // bool | (προαιρετικό)
	includeByEmailDomain := true // bool | (προαιρετικό)
	sso := "sso_example" // string | (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).TenantId(tenantId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Σφάλμα κατά την κλήση του `ModerationAPI.GetPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Πλήρης απάντηση HTTP: %v\n", r)
	}
	// απάντηση από `GetPreBanSummary`: PreBanSummary
	fmt.Fprintf(os.Stdout, "Απάντηση από `ModerationAPI.GetPreBanSummary`: %v\n", resp)
}
[inline-code-end]