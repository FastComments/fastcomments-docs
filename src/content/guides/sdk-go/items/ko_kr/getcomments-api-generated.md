## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| anonUserId | string | query | 아니요 |  |
| contextUserId | string | query | 아니요 |  |
| hashTag | string | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| direction | string | query | 아니요 |  |
| fromDate | integer | query | 아니요 |  |
| toDate | integer | query | 아니요 |  |

## 응답

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## 예제

[inline-code-attrs-start title = 'GetComments 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (선택 사항)
	limit := int32(56) // int32 |  (선택 사항)
	skip := int32(56) // int32 |  (선택 사항)
	asTree := true // bool |  (선택 사항)
	skipChildren := int32(56) // int32 |  (선택 사항)
	limitChildren := int32(56) // int32 |  (선택 사항)
	maxTreeDepth := int32(56) // int32 |  (선택 사항)
	urlId := "urlId_example" // string |  (선택 사항)
	userId := "userId_example" // string |  (선택 사항)
	anonUserId := "anonUserId_example" // string |  (선택 사항)
	contextUserId := "contextUserId_example" // string |  (선택 사항)
	hashTag := "hashTag_example" // string |  (선택 사항)
	parentId := "parentId_example" // string |  (선택 사항)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (선택 사항)
	fromDate := int64(789) // int64 |  (선택 사항)
	toDate := int64(789) // int64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetComments`의 응답: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]