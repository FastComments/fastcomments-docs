## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | 예 |  |
| tenantId | string | query | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## 예제

[inline-code-attrs-start title = 'DeleteHashTag 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  (선택 사항)
	deleteHashTagRequest := *openapiclient.NewDeleteHashTagRequest() // DeleteHashTagRequest |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteHashTag(context.Background(), tag).TenantId(tenantId).DeleteHashTagRequest(deleteHashTagRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `DeleteHashTag`의 응답: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteHashTag`: %v\n", resp)
}
[inline-code-end]