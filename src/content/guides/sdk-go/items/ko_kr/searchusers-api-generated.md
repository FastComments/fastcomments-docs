## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니오 |  |
| mentionGroupIds | array | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| searchSection | string | query | 아니오 |  |

## 응답

반환: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## 예제

[inline-code-attrs-start title = 'SearchUsers 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	usernameStartsWith := "usernameStartsWith_example" // string |  (선택)
	mentionGroupIds := []string{"Inner_example"} // []string |  (선택)
	sso := "sso_example" // string |  (선택)
	searchSection := "searchSection_example" // string |  (선택)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SearchUsers`의 응답: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]

---