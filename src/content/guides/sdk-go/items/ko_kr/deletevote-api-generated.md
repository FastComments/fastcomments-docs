## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| editKey | string | query | No |  |

## 응답

반환: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_vote_delete_response.go)

## 예제

[inline-code-attrs-start title = 'DeleteVote 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	editKey := "editKey_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteVote(context.Background(), id).TenantId(tenantId).EditKey(editKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteVote``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteVote`의 응답: VoteDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteVote`: %v\n", resp)
}
[inline-code-end]