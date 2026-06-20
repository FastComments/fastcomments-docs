## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| sendEmails | boolean | query | Nej |  |
| populateNotifications | boolean | query | Nej |  |

## Respons

Returnerer: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_save_comment_response.go)

## Eksempel

[inline-code-attrs-start title = 'SaveComment eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createCommentParams := *openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example") // CreateCommentParams | 
	isLive := true // bool |  (valgfri)
	doSpamCheck := true // bool |  (valgfri)
	sendEmails := true // bool |  (valgfri)
	populateNotifications := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveComment(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `SaveComment`: APISaveCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveComment`: %v\n", resp)
}
[inline-code-end]