req
tenantId
urlId

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| page | integer | query | 否 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |
| skip | integer | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| countChildren | boolean | query | 否 |  |
| fetchPageForCommentId | string | query | 否 |  |
| includeConfig | boolean | query | 否 |  |
| countAll | boolean | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| modules | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeNotificationCount | boolean | query | 否 |  |
| asTree | boolean | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |
| parentId | string | query | 否 |  |
| searchText | string | query | 否 |  |
| hashTags | array | query | 否 |  |
| userId | string | query | 否 |  |
| customConfigStr | string | query | 否 |  |
| afterCommentId | string | query | 否 |  |
| beforeCommentId | string | query | 否 |  |

## 回應

回傳: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetCommentsPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (可選)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (可選)
	sso := "sso_example" // string |  (可選)
	skip := int32(56) // int32 |  (可選)
	skipChildren := int32(56) // int32 |  (可選)
	limit := int32(56) // int32 |  (可選)
	limitChildren := int32(56) // int32 |  (可選)
	countChildren := true // bool |  (可選)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (可選)
	includeConfig := true // bool |  (可選)
	countAll := true // bool |  (可選)
	includei10n := true // bool |  (可選)
	locale := "locale_example" // string |  (可選)
	modules := "modules_example" // string |  (可選)
	isCrawler := true // bool |  (可選)
	includeNotificationCount := true // bool |  (可選)
	asTree := true // bool |  (可選)
	maxTreeDepth := int32(56) // int32 |  (可選)
	useFullTranslationIds := true // bool |  (可選)
	parentId := "parentId_example" // string |  (可選)
	searchText := "searchText_example" // string |  (可選)
	hashTags := []string{"Inner_example"} // []string |  (可選)
	userId := "userId_example" // string |  (可選)
	customConfigStr := "customConfigStr_example" // string |  (可選)
	afterCommentId := "afterCommentId_example" // string |  (可選)
	beforeCommentId := "beforeCommentId_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `GetCommentsPublic` 的回應: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]