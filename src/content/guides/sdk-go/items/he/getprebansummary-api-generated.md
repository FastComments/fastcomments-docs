## פרמטרים

| שם | טיפוס | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| includeByUserIdAndEmail | boolean | query | לא |  |
| includeByIP | boolean | query | לא |  |
| includeByEmailDomain | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetPreBanSummary'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool |  (אופציונלי)
	includeByIP := true // bool |  (אופציונלי)
	includeByEmailDomain := true // bool |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetPreBanSummary`: PreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetPreBanSummary`: %v\n", resp)
}
[inline-code-end]