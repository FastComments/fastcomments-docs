## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | integer | query | לא |  |
| limit | integer | query | לא |  |
| skip | integer | query | לא |  |
| asTree | boolean | query | לא |  |
| skipChildren | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |
| contextUserId | string | query | לא |  |
| hashTag | string | query | לא |  |
| parentId | string | query | לא |  |
| direction | string | query | לא |  |

## תגובה

מחזיר: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (אופציונלי)
	limit := int32(56) // int32 |  (אופציונלי)
	skip := int32(56) // int32 |  (אופציונלי)
	asTree := true // bool |  (אופציונלי)
	skipChildren := int32(56) // int32 |  (אופציונלי)
	limitChildren := int32(56) // int32 |  (אופציונלי)
	maxTreeDepth := int32(56) // int32 |  (אופציונלי)
	urlId := "urlId_example" // string |  (אופציונלי)
	userId := "userId_example" // string |  (אופציונלי)
	anonUserId := "anonUserId_example" // string |  (אופציונלי)
	contextUserId := "contextUserId_example" // string |  (אופציונלי)
	hashTag := "hashTag_example" // string |  (אופציונלי)
	parentId := "parentId_example" // string |  (אופציונלי)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]

---