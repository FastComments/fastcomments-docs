## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
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

## Odziv

Vrne: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (neobvezno)
	limit := int32(56) // int32 |  (neobvezno)
	skip := int32(56) // int32 |  (neobvezno)
	asTree := true // bool |  (neobvezno)
	skipChildren := int32(56) // int32 |  (neobvezno)
	limitChildren := int32(56) // int32 |  (neobvezno)
	maxTreeDepth := int32(56) // int32 |  (neobvezno)
	urlId := "urlId_example" // string |  (neobvezno)
	userId := "userId_example" // string |  (neobvezno)
	anonUserId := "anonUserId_example" // string |  (neobvezno)
	contextUserId := "contextUserId_example" // string |  (neobvezno)
	hashTag := "hashTag_example" // string |  (neobvezno)
	parentId := "parentId_example" // string |  (neobvezno)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (neobvezno)
	fromDate := int64(789) // int64 |  (neobvezno)
	toDate := int64(789) // int64 |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]