## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Antwort

Gibt zurück: [`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## Beispiel

[inline-code-attrs-start title = 'GetPreBanSummary Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool |  (optional)
	includeByIP := true // bool |  (optional)
	includeByEmailDomain := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetPreBanSummary`: PreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetPreBanSummary`: %v\n", resp)
}
[inline-code-end]