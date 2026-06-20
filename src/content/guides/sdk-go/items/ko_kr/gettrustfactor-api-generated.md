## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| userId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_trust_factor_response.go)

## 예제

[inline-code-attrs-start title = 'GetTrustFactor 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetTrustFactor(context.Background()).UserId(userId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTrustFactor`의 응답: GetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetTrustFactor`: %v\n", resp)
}
[inline-code-end]