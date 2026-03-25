## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| state | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| limit | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## 例

[inline-code-attrs-start title = 'GetTickets の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (任意)
	state := float64(1.2) // float64 |  (任意)
	skip := float64(1.2) // float64 |  (任意)
	limit := float64(1.2) // float64 |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]