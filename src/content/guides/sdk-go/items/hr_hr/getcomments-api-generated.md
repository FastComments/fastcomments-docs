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

## Primjer

[inline-code-attrs-start title = 'GetComments Primjer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (neobavezno)
	limit := int32(56) // int32 |  (neobavezno)
	skip := int32(56) // int32 |  (neobavezno)
	asTree := true // bool |  (neobavezno)
	skipChildren := int32(56) // int32 |  (neobavezno)
	limitChildren := int32(56) // int32 |  (neobavezno)
	maxTreeDepth := int32(56) // int32 |  (neobavezno)
	urlId := "urlId_example" // string |  (neobavezno)
	userId := "userId_example" // string |  (neobavezno)
	anonUserId := "anonUserId_example" // string |  (neobavezno)
	contextUserId := "contextUserId_example" // string |  (neobavezno)
	hashTag := "hashTag_example" // string |  (neobavezno)
	parentId := "parentId_example" // string |  (neobavezno)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (neobavezno)

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