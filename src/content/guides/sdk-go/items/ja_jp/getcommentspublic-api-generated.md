req
tenantId
urlId

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
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

返却: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## 例

[inline-code-attrs-start title = 'GetCommentsPublic の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 
	page := int32(56) // int32 |  (任意)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (任意)
	sso := "sso_example" // string |  (任意)
	skip := int32(56) // int32 |  (任意)
	skipChildren := int32(56) // int32 |  (任意)
	limit := int32(56) // int32 |  (任意)
	limitChildren := int32(56) // int32 |  (任意)
	countChildren := true // bool |  (任意)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (任意)
	includeConfig := true // bool |  (任意)
	countAll := true // bool |  (任意)
	includei10n := true // bool |  (任意)
	locale := "locale_example" // string |  (任意)
	modules := "modules_example" // string |  (任意)
	isCrawler := true // bool |  (任意)
	includeNotificationCount := true // bool |  (任意)
	asTree := true // bool |  (任意)
	maxTreeDepth := int32(56) // int32 |  (任意)
	useFullTranslationIds := true // bool |  (任意)
	parentId := "parentId_example" // string |  (任意)
	searchText := "searchText_example" // string |  (任意)
	hashTags := []string{"Inner_example"} // []string |  (任意)
	userId := "userId_example" // string |  (任意)
	customConfigStr := "customConfigStr_example" // string |  (任意)
	afterCommentId := "afterCommentId_example" // string |  (任意)
	beforeCommentId := "beforeCommentId_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsPublic` のレスポンス: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]