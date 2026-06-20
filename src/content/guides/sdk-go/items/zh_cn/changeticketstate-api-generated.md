## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_ticket_state_response.go)

## 示例

[inline-code-attrs-start title = 'ChangeTicketState 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// 来自 `ChangeTicketState` 的响应: ChangeTicketStateResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ChangeTicketState`: %v\n", resp)
}
[inline-code-end]