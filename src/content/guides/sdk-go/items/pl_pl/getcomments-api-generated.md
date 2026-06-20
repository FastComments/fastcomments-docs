## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| page | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| skip | integer | query | Nie |  |
| asTree | boolean | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |
| contextUserId | string | query | Nie |  |
| hashTag | string | query | Nie |  |
| parentId | string | query | Nie |  |
| direction | string | query | Nie |  |
| fromDate | integer | query | Nie |  |
| toDate | integer | query | Nie |  |

## Odpowiedź

Zwraca: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opcjonalne)
	limit := int32(56) // int32 |  (opcjonalne)
	skip := int32(56) // int32 |  (opcjonalne)
	asTree := true // bool |  (opcjonalne)
	skipChildren := int32(56) // int32 |  (opcjonalne)
	limitChildren := int32(56) // int32 |  (opcjonalne)
	maxTreeDepth := int32(56) // int32 |  (opcjonalne)
	urlId := "urlId_example" // string |  (opcjonalne)
	userId := "userId_example" // string |  (opcjonalne)
	anonUserId := "anonUserId_example" // string |  (opcjonalne)
	contextUserId := "contextUserId_example" // string |  (opcjonalne)
	hashTag := "hashTag_example" // string |  (opcjonalne)
	parentId := "parentId_example" // string |  (opcjonalne)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcjonalne)
	fromDate := int64(789) // int64 |  (opcjonalne)
	toDate := int64(789) // int64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]