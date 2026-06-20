## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| skip | integer | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |
| contextUserId | string | query | いいえ |  |
| hashTag | string | query | いいえ |  |
| parentId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| fromDate | integer | query | いいえ |  |
| toDate | integer | query | いいえ |  |

## レスポンス

返却: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## 例

[inline-code-attrs-start title = 'GetComments の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (オプション)
	limit := int32(56) // int32 |  (オプション)
	skip := int32(56) // int32 |  (オプション)
	asTree := true // bool |  (オプション)
	skipChildren := int32(56) // int32 |  (オプション)
	limitChildren := int32(56) // int32 |  (オプション)
	maxTreeDepth := int32(56) // int32 |  (オプション)
	urlId := "urlId_example" // string |  (オプション)
	userId := "userId_example" // string |  (オプション)
	anonUserId := "anonUserId_example" // string |  (オプション)
	contextUserId := "contextUserId_example" // string |  (オプション)
	hashTag := "hashTag_example" // string |  (オプション)
	parentId := "parentId_example" // string |  (オプション)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (オプション)
	fromDate := int64(789) // int64 |  (オプション)
	toDate := int64(789) // int64 |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]