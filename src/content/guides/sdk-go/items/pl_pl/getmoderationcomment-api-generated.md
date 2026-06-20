## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| includeEmail | boolean | query | Nie |  |
| includeIP | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_comment_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetModerationComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeEmail := true // bool |  (opcjonalne)
	includeIP := true // bool |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationComment(context.Background(), commentId).IncludeEmail(includeEmail).IncludeIP(includeIP).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetModerationComment`: ModerationAPICommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationComment`: %v\n", resp)
}
[inline-code-end]