## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |

## レスポンス

戻り値: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_ticket_200_response.go)

## 例

[inline-code-attrs-start title = 'GetTicket の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	userId := "userId_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTicket(context.Background(), id).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTicket``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTicket` のレスポンス: GetTicket200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTicket`: %v\n", resp)
}
[inline-code-end]