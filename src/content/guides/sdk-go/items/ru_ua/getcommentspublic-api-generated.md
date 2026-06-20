req
tenantId
urlId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |

## Ответ

Возвращает: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Пример

[inline-code-attrs-start title = 'Пример GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (необязательно)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (необязательно)
	sso := "sso_example" // string |  (необязательно)
	skip := int32(56) // int32 |  (необязательно)
	skipChildren := int32(56) // int32 |  (необязательно)
	limit := int32(56) // int32 |  (необязательно)
	limitChildren := int32(56) // int32 |  (необязательно)
	countChildren := true // bool |  (необязательно)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (необязательно)
	includeConfig := true // bool |  (необязательно)
	countAll := true // bool |  (необязательно)
	includei10n := true // bool |  (необязательно)
	locale := "locale_example" // string |  (необязательно)
	modules := "modules_example" // string |  (необязательно)
	isCrawler := true // bool |  (необязательно)
	includeNotificationCount := true // bool |  (необязательно)
	asTree := true // bool |  (необязательно)
	maxTreeDepth := int32(56) // int32 |  (необязательно)
	useFullTranslationIds := true // bool |  (необязательно)
	parentId := "parentId_example" // string |  (необязательно)
	searchText := "searchText_example" // string |  (необязательно)
	hashTags := []string{"Inner_example"} // []string |  (необязательно)
	userId := "userId_example" // string |  (необязательно)
	customConfigStr := "customConfigStr_example" // string |  (необязательно)
	afterCommentId := "afterCommentId_example" // string |  (необязательно)
	beforeCommentId := "beforeCommentId_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]