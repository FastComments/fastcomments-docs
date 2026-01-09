## פרמטרים

| Name | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| pageSize | integer | query | לא |  |
| afterId | string | query | לא |  |
| includeContext | boolean | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| includeTranslations | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notifications_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetUserNotifications'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	pageSize := int32(56) // int32 |  (אופציונלי)
	afterId := "afterId_example" // string |  (אופציונלי)
	includeContext := true // bool |  (אופציונלי)
	afterCreatedAt := int64(789) // int64 |  (אופציונלי)
	unreadOnly := true // bool |  (אופציונלי)
	dmOnly := true // bool |  (אופציונלי)
	noDm := true // bool |  (אופציונלי)
	includeTranslations := true // bool |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotifications(context.Background()).TenantId(tenantId).PageSize(pageSize).AfterId(afterId).IncludeContext(includeContext).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).IncludeTranslations(includeTranslations).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetUserNotifications`: GetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotifications`: %v\n", resp)
}
[inline-code-end]

---