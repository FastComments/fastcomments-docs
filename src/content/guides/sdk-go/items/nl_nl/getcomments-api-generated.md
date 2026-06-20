## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| skip | integer | query | Nee |  |
| asTree | boolean | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| urlId | string | query | Nee |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |
| contextUserId | string | query | Nee |  |
| hashTag | string | query | Nee |  |
| parentId | string | query | Nee |  |
| direction | string | query | Nee |  |
| fromDate | integer | query | Nee |  |
| toDate | integer | query | Nee |  |

## Respons

Retourneert: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetComments Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optioneel)
	limit := int32(56) // int32 |  (optioneel)
	skip := int32(56) // int32 |  (optioneel)
	asTree := true // bool |  (optioneel)
	skipChildren := int32(56) // int32 |  (optioneel)
	limitChildren := int32(56) // int32 |  (optioneel)
	maxTreeDepth := int32(56) // int32 |  (optioneel)
	urlId := "urlId_example" // string |  (optioneel)
	userId := "userId_example" // string |  (optioneel)
	anonUserId := "anonUserId_example" // string |  (optioneel)
	contextUserId := "contextUserId_example" // string |  (optioneel)
	hashTag := "hashTag_example" // string |  (optioneel)
	parentId := "parentId_example" // string |  (optioneel)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optioneel)
	fromDate := int64(789) // int64 |  (optioneel)
	toDate := int64(789) // int64 |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwoord van `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]

---