## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| isLive | boolean | query | Hayır |  |
| doSpamCheck | boolean | query | Hayır |  |
| sendEmails | boolean | query | Hayır |  |
| populateNotifications | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comment_200_response.go)

## Örnek

[inline-code-attrs-start title = 'SaveComment Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createCommentParams := *openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example") // CreateCommentParams | 
	isLive := true // bool |  (isteğe bağlı)
	doSpamCheck := true // bool |  (isteğe bağlı)
	sendEmails := true // bool |  (isteğe bağlı)
	populateNotifications := true // bool |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveComment(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SaveComment`'tan gelen yanıt: SaveComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveComment`: %v\n", resp)
}
[inline-code-end]