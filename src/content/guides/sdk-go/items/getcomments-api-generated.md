## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
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

## Response

Returns: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Example

[inline-code-attrs-start title = 'GetComments Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]
