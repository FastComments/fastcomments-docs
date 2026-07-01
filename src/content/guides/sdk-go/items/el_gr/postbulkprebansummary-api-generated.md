## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| includeByUserIdAndEmail | boolean | query | Όχι |  |
| includeByIP | boolean | query | Όχι |  |
| includeByEmailDomain | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_bulk_pre_ban_summary.go)

## Example

[inline-code-attrs-start title = 'Παράδειγμα PostBulkPreBanSummary'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	bulkPreBanParams := *openapiclient.NewBulkPreBanParams([]string{"CommentIds_example"}) // BulkPreBanParams | 
	includeByUserIdAndEmail := true // bool |  (προαιρετικό)
	includeByIP := true // bool |  (προαιρετικό)
	includeByEmailDomain := true // bool |  (πρε

[inline-code-end]