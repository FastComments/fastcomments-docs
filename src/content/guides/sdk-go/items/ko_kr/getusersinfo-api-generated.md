테넌트의 대량 사용자 정보. userIds를 받아 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서, presence 이벤트로 방금 나타난 사용자의 정보를 보강하기 위해 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| ids | string | 쿼리 | 예 | 쉼표로 구분된 userIds. |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_info_response.go)

## 예제

[inline-code-attrs-start title = 'GetUsersInfo 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	ids := "ids_example" // string | 쉼표로 구분된 userIds.

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUsersInfo(context.Background(), tenantId).Ids(ids).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUsersInfo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUsersInfo`의 응답: PageUsersInfoResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUsersInfo`: %v\n", resp)
}
[inline-code-end]