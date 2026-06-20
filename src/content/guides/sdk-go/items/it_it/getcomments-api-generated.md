## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Risposta

Restituisce: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opzionale)
	limit := int32(56) // int32 |  (opzionale)
	skip := int32(56) // int32 |  (opzionale)
	asTree := true // bool |  (opzionale)
	skipChildren := int32(56) // int32 |  (opzionale)
	limitChildren := int32(56) // int32 |  (opzionale)
	maxTreeDepth := int32(56) // int32 |  (opzionale)
	urlId := "urlId_example" // string |  (opzionale)
	userId := "userId_example" // string |  (opzionale)
	anonUserId := "anonUserId_example" // string |  (opzionale)
	contextUserId := "contextUserId_example" // string |  (opzionale)
	hashTag := "hashTag_example" // string |  (opzionale)
	parentId := "parentId_example" // string |  (opzionale)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opzionale)
	fromDate := int64(789) // int64 |  (opzionale)
	toDate := int64(789) // int64 |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]