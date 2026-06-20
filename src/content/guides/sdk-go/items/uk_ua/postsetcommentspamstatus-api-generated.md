## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| spam | boolean | query | Ні |  |
| permNotSpam | boolean | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад PostSetCommentSpamStatus'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	spam := true // bool |  (необов'язково)
	permNotSpam := true // bool |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostSetCommentSpamStatus(context.Background(), commentId).Spam(spam).PermNotSpam(permNotSpam).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostSetCommentSpamStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `PostSetCommentSpamStatus`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostSetCommentSpamStatus`: %v\n", resp)
}
[inline-code-end]