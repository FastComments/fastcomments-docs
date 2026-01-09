## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| isLive | boolean | query | Nej |  |
| doSpamCheck | boolean | query | Nej |  |
| sendEmails | boolean | query | Nej |  |
| populateNotifications | boolean | query | Nej |  |

## Respons

Returnerer: `[]SaveComment200Response`

## Eksempel

[inline-code-attrs-start title = 'SaveCommentsBulk Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isLive := true // bool |  (valgfri)
	doSpamCheck := true // bool |  (valgfri)
	sendEmails := true // bool |  (valgfri)
	populateNotifications := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.SaveCommentsBulk(context.Background()).TenantId(tenantId).CreateCommentParams(createCommentParams).IsLive(isLive).DoSpamCheck(doSpamCheck).SendEmails(sendEmails).PopulateNotifications(populateNotifications).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.SaveCommentsBulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `SaveCommentsBulk`: []SaveComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.SaveCommentsBulk`: %v\n", resp)
}
[inline-code-end]

---