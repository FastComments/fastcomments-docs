## Parametre

| Name | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| skip | integer | query | Nej |  |
| asTree | boolean | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |
| contextUserId | string | query | Nej |  |
| hashTag | string | query | Nej |  |
| parentId | string | query | Nej |  |
| direction | string | query | Nej |  |
| fromDate | integer | query | Nej |  |
| toDate | integer | query | Nej |  |

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetComments Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (valgfri)
	limit := int32(56) // int32 |  (valgfri)
	skip := int32(56) // int32 |  (valgfri)
	asTree := true // bool |  (valgfri)
	skipChildren := int32(56) // int32 |  (valgfri)
	limitChildren := int32(56) // int32 |  (valgfri)
	maxTreeDepth := int32(56) // int32 |  (valgfri)
	urlId := "urlId_example" // string |  (valgfri)
	userId := "userId_example" // string |  (valgfri)
	anonUserId := "anonUserId_example" // string |  (valgfri)
	contextUserId := "contextUserId_example" // string |  (valgfri)
	hashTag := "hashTag_example" // string |  (valgfri)
	parentId := "parentId_example" // string |  (valgfri)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (valgfri)
	fromDate := int64(789) // int64 |  (valgfri)
	toDate := int64(789) // int64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]