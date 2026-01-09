## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| commentId | string | 경로 | 예 |  |
| editKey | string | 쿼리 | 아니오 |  |
| sso | string | 쿼리 | 아니오 |  |

## 응답

반환: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetCommentText 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	commentId := "commentId_example" // string | 
	editKey := "editKey_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentText(context.Background(), tenantId, commentId).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentText`의 응답: GetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentText`: %v\n", resp)
}
[inline-code-end]

---