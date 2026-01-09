## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| isLive | boolean | query | לא |  |
| doSpamCheck | boolean | query | לא |  |
| sendEmails | boolean | query | לא |  |
| populateNotifications | boolean | query | לא |  |

## תגובה

מחזיר: `[]SaveComment200Response`

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-SaveCommentsBulk'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createCommentParams := []openapiclient.CreateCommentParams{*openapiclient.NewCreateCommentParams("CommenterName_example", "Comment_example", "Url_example", "UrlId_example", "Locale_example")} // []CreateCommentParams | 
	isLive := true // bool |  (אופציונלי)
	doSpamCheck := true // bool |  (אופציונלי)
	sendEmails := true // bool |  (אופציונלי)
	populateNotifications := true // bool |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveCommentsBulk(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveCommentsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`SaveCommentsBulk`: []SaveComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveCommentsBulk`: %v\n", resp)
}
[inline-code-end]