테넌트의 페이지를 나열합니다. FChat 데스크톱 클라이언트가 룸 목록을 채우기 위해 사용합니다.
각 페이지에 대해 해결된 커스텀 구성에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 선택적 대소문자 구분 없는 제목 접두사 필터입니다. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | No | true일 경우 댓글이 최소 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## 예제

[inline-code-attrs-start title = 'GetPagesPublic 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. (선택 사항)
	limit := int32(56) // int32 | 1..200, 기본값 50 (선택 사항)
	q := "q_example" // string | 선택적 대소문자 구분 없는 제목 접두사 필터입니다. (선택 사항)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). (선택 사항)
	hasComments := true // bool | true일 경우 댓글이 최소 하나 이상 있는 페이지만 반환합니다. (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPagesPublic`의 응답: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]