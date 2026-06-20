## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | number | query | 아니요 |  |

## 응답

반환: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_hash_tags_response.go)

## 예제

[inline-code-attrs-start title = 'GetHashTags 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetHashTags(context.Background()).TenantId(tenantId).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetHashTags``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetHashTags`의 응답: GetHashTagsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetHashTags`: %v\n", resp)
}
[inline-code-end]