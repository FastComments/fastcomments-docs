## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Response

Returnerer: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_post_remove_comment_response.go)

## Eksempel

[inline-code-attrs-start title = 'PostRemoveComment Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostRemoveComment(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostRemoveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `PostRemoveComment`: PostRemoveCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostRemoveComment`: %v\n", resp)
}
[inline-code-end]

---