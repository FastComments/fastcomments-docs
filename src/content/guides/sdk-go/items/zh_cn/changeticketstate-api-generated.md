## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## 响应

返回: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_ticket_state_200_response.go)

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
	tenantId := "tenantId_example" // 字符串 | 
	userId := "userId_example" // 字符串 | 
	id := "id_example" // 字符串 | 
	changeTicketStateBody := *openapiclient.NewChangeTicketStateBody(int32(123)) // ChangeTicketStateBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ChangeTicketState(context.Background(), id).TenantId(tenantId).UserId(userId).ChangeTicketStateBody(changeTicketStateBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ChangeTicketState``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `ChangeTicketState` 的响应: ChangeTicketState200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ChangeTicketState`: %v\n", resp)
}
[inline-code-end]