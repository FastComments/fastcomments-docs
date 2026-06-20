---
## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | pot | Da |  |
| sso | string | poizvedba | Ne |  |

## Odgovor

Vrne: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comment_text_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetModerationCommentText'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationCommentText(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationCommentText``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetModerationCommentText`: GetCommentTextResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationCommentText`: %v\n", resp)
}
[inline-code-end]

---