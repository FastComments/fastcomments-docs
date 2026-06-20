## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_page_search_response.go)

## 예제

[inline-code-attrs-start title = 'GetSearchPages 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchPages(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchPages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSearchPages`의 응답: ModerationPageSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchPages`: %v\n", resp)
}
[inline-code-end]