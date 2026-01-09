## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| skip | integer | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| urlId | string | query | Όχι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |
| contextUserId | string | query | Όχι |  |
| hashTag | string | query | Όχι |  |
| parentId | string | query | Όχι |  |
| direction | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (προαιρετικό)
	limit := int32(56) // int32 |  (προαιρετικό)
	skip := int32(56) // int32 |  (προαιρετικό)
	asTree := true // bool |  (προαιρετικό)
	skipChildren := int32(56) // int32 |  (προαιρετικό)
	limitChildren := int32(56) // int32 |  (προαιρετικό)
	maxTreeDepth := int32(56) // int32 |  (προαιρετικό)
	urlId := "urlId_example" // string |  (προαιρετικό)
	userId := "userId_example" // string |  (προαιρετικό)
	anonUserId := "anonUserId_example" // string |  (προαιρετικό)
	contextUserId := "contextUserId_example" // string |  (προαιρετικό)
	hashTag := "hashTag_example" // string |  (προαιρετικό)
	parentId := "parentId_example" // string |  (προαιρετικό)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]

---