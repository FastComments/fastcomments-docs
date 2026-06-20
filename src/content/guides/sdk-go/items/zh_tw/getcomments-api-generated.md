## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| page | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| skip | integer | query | 否 |  |
| asTree | boolean | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |
| contextUserId | string | query | 否 |  |
| hashTag | string | query | 否 |  |
| parentId | string | query | 否 |  |
| direction | string | query | 否 |  |
| fromDate | integer | query | 否 |  |
| toDate | integer | query | 否 |  |

## 回應

回傳: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## 範例

[inline-code-attrs-start title = 'GetComments 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (可選)
	limit := int32(56) // int32 |  (可選)
	skip := int32(56) // int32 |  (可選)
	asTree := true // bool |  (可選)
	skipChildren := int32(56) // int32 |  (可選)
	limitChildren := int32(56) // int32 |  (可選)
	maxTreeDepth := int32(56) // int32 |  (可選)
	urlId := "urlId_example" // string |  (可選)
	userId := "userId_example" // string |  (可選)
	anonUserId := "anonUserId_example" // string |  (可選)
	contextUserId := "contextUserId_example" // string |  (可選)
	hashTag := "hashTag_example" // string |  (可選)
	parentId := "parentId_example" // string |  (可選)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (可選)
	fromDate := int64(789) // int64 |  (可選)
	toDate := int64(789) // int64 |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `GetComments` 的回應: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]