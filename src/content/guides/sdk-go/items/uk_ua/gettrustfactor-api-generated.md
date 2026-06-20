---
## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| userId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_trust_factor_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetTrustFactor'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetTrustFactor(context.Background()).UserId(userId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetTrustFactor`: GetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetTrustFactor`: %v\n", resp)
}
[inline-code-end]

---