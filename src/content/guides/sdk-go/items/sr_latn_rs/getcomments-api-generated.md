## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| skip | integer | query | Ne |  |
| asTree | boolean | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |
| contextUserId | string | query | Ne |  |
| hashTag | string | query | Ne |  |
| parentId | string | query | Ne |  |
| direction | string | query | Ne |  |

## Odgovor

Vraća: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Primer

[inline-code-attrs-start title = 'Primer za GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opciono)
	limit := int32(56) // int32 |  (opciono)
	skip := int32(56) // int32 |  (opciono)
	asTree := true // bool |  (opciono)
	skipChildren := int32(56) // int32 |  (opciono)
	limitChildren := int32(56) // int32 |  (opciono)
	maxTreeDepth := int32(56) // int32 |  (opciono)
	urlId := "urlId_example" // string |  (opciono)
	userId := "userId_example" // string |  (opciono)
	anonUserId := "anonUserId_example" // string |  (opciono)
	contextUserId := "contextUserId_example" // string |  (opciono)
	hashTag := "hashTag_example" // string |  (opciono)
	parentId := "parentId_example" // string |  (opciono)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opciono)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]