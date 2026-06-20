## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| isLive | boolean | query | לא |  |
| doSpamCheck | boolean | query | לא |  |
| sendEmails | boolean | query | לא |  |
| populateNotifications | boolean | query | לא |  |

## תגובה

מחזיר: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_save_comment_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-SaveComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isLive := true // bool |  (אופציונלי)
	doSpamCheck := true // bool |  (אופציונלי)
	sendEmails := true // bool |  (אופציונלי)
	populateNotifications := true // bool |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveComment(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`SaveComment`: APISaveCommentResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveComment`: %v\n", resp)
}
[inline-code-end]