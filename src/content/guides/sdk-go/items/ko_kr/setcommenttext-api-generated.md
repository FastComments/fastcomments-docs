## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| commentId | string | 경로 | 예 |  |
| broadcastId | string | 쿼리 | 예 |  |
| editKey | string | 쿼리 | 아니요 |  |
| sso | string | 쿼리 | 아니요 |  |

## 응답

반환: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_comment_text_200_response.go)

## 예제

[inline-code-attrs-start title = 'SetCommentText 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string | 
	commentTextUpdateRequest := *openapiclient.NewCommentTextUpdateRequest("Comment_example") // CommentTextUpdateRequest | 
	editKey := "editKey_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SetCommentText(context.Background(), tenantId, commentId).BroadcastId(broadcastId).CommentTextUpdateRequest(commentTextUpdateRequest).EditKey(editKey).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SetCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SetCommentText`의 응답: SetCommentText200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SetCommentText`: %v\n", resp)
}
[inline-code-end]