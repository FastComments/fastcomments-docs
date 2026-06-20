---
## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | לא |  |
| includeByIP | boolean | query | לא |  |
| includeByEmailDomain | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_pre_ban_summary.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-PostBulkPreBanSummary'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	bulkPreBanParams := *openapiclient.NewBulkPreBanParams([]string{"CommentIds_example"}) // BulkPreBanParams | 
	includeByUserIdAndEmail := true // bool |  (אופציונלי)
	includeByIP := true // bool |  (אופציונלי)
	includeByEmailDomain := true // bool |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBulkPreBanSummary(context.Background()).BulkPreBanParams(bulkPreBanParams).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBulkPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`PostBulkPreBanSummary`: BulkPreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBulkPreBanSummary`: %v\n", resp)
}
[inline-code-end]

---