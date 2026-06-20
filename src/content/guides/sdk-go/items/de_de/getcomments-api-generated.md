## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| skip | integer | query | Nein |  |
| asTree | boolean | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |
| contextUserId | string | query | Nein |  |
| hashTag | string | query | Nein |  |
| parentId | string | query | Nein |  |
| direction | string | query | Nein |  |
| fromDate | integer | query | Nein |  |
| toDate | integer | query | Nein |  |

## Antwort

Gibt zurück: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetComments Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optional)
	limit := int32(56) // int32 |  (optional)
	skip := int32(56) // int32 |  (optional)
	asTree := true // bool |  (optional)
	skipChildren := int32(56) // int32 |  (optional)
	limitChildren := int32(56) // int32 |  (optional)
	maxTreeDepth := int32(56) // int32 |  (optional)
	urlId := "urlId_example" // string |  (optional)
	userId := "userId_example" // string |  (optional)
	anonUserId := "anonUserId_example" // string |  (optional)
	contextUserId := "contextUserId_example" // string |  (optional)
	hashTag := "hashTag_example" // string |  (optional)
	parentId := "parentId_example" // string |  (optional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optional)
	fromDate := int64(789) // int64 |  (optional)
	toDate := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]