---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_ticket_state_200_response.go)

## 範例

[inline-code-attrs-start title = 'ChangeTicketState 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string | 
	id := "id_example" // string | 
	changeTicketStateBody := *openapiclient.NewChangeTicketStateBody(int32(123)) // ChangeTicketStateBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ChangeTicketState(context.Background(), id).TenantId(tenantId).UserId(userId).ChangeTicketStateBody(changeTicketStateBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ChangeTicketState``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `ChangeTicketState` 的回應：ChangeTicketState200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ChangeTicketState`: %v\n", resp)
}
[inline-code-end]

---