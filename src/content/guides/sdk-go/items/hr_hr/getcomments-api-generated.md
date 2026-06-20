## Parametri

| Name | Type | Location | Required | Description |
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
| fromDate | integer | query | Ne |  |
| toDate | integer | query | Ne |  |

## Odgovor

Vraća: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Primjer

[inline-code-attrs-start title = 'Primjer GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	fromDate := int64(789) // int64 |  (neobavezno)
	toDate := int64(789) // int64 |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]