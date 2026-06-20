req
tenantId
urlId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| page | integer | query | いいえ |  |
| direction | string | query | いいえ |  |
| sso | string | query | いいえ |  |
| skip | integer | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| countChildren | boolean | query | いいえ |  |
| fetchPageForCommentId | string | query | いいえ |  |
| includeConfig | boolean | query | いいえ |  |
| countAll | boolean | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| modules | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeNotificationCount | boolean | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |
| parentId | string | query | いいえ |  |
| searchText | string | query | いいえ |  |
| hashTags | array | query | いいえ |  |
| userId | string | query | いいえ |  |
| customConfigStr | string | query | いいえ |  |
| afterCommentId | string | query | いいえ |  |
| beforeCommentId | string | query | いいえ |  |

## レスポンス

返却値: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## 例

[inline-code-attrs-start title = 'GetCommentsPublic の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	page := int32(56) // int32 |  (オプション)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (オプション)
	sso := "sso_example" // string |  (オプション)
	skip := int32(56) // int32 |  (オプション)
	skipChildren := int32(56) // int32 |  (オプション)
	limit := int32(56) // int32 |  (オプション)
	limitChildren := int32(56) // int32 |  (オプション)
	countChildren := true // bool |  (オプション)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (オプション)
	includeConfig := true // bool |  (オプション)
	countAll := true // bool |  (オプション)
	includei10n := true // bool |  (オプション)
	locale := "locale_example" // string |  (オプション)
	modules := "modules_example" // string |  (オプション)
	isCrawler := true // bool |  (オプション)
	includeNotificationCount := true // bool |  (オプション)
	asTree := true // bool |  (オプション)
	maxTreeDepth := int32(56) // int32 |  (オプション)
	useFullTranslationIds := true // bool |  (オプション)
	parentId := "parentId_example" // string |  (オプション)
	searchText := "searchText_example" // string |  (オプション)
	hashTags := []string{"Inner_example"} // []string |  (オプション)
	userId := "userId_example" // string |  (オプション)
	customConfigStr := "customConfigStr_example" // string |  (オプション)
	afterCommentId := "afterCommentId_example" // string |  (オプション)
	beforeCommentId := "beforeCommentId_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsPublic`のレスポンス: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]