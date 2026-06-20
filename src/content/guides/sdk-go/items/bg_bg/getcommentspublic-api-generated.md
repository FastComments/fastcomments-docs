req
tenantId
urlId

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Не |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |
| skip | integer | query | Не |  |
| skipChildren | integer | query | Не |  |
| limit | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| countChildren | boolean | query | Не |  |
| fetchPageForCommentId | string | query | Не |  |
| includeConfig | boolean | query | Не |  |
| countAll | boolean | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| modules | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeNotificationCount | boolean | query | Не |  |
| asTree | boolean | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| useFullTranslationIds | boolean | query | Не |  |
| parentId | string | query | Не |  |
| searchText | string | query | Не |  |
| hashTags | array | query | Не |  |
| userId | string | query | Не |  |
| customConfigStr | string | query | Не |  |
| afterCommentId | string | query | Не |  |
| beforeCommentId | string | query | Не |  |

## Отговор

Връща: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (по избор)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (по избор)
	sso := "sso_example" // string |  (по избор)
	skip := int32(56) // int32 |  (по избор)
	skipChildren := int32(56) // int32 |  (по избор)
	limit := int32(56) // int32 |  (по избор)
	limitChildren := int32(56) // int32 |  (по избор)
	countChildren := true // bool |  (по избор)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (по избор)
	includeConfig := true // bool |  (по избор)
	countAll := true // bool |  (по избор)
	includei10n := true // bool |  (по избор)
	locale := "locale_example" // string |  (по избор)
	modules := "modules_example" // string |  (по избор)
	isCrawler := true // bool |  (по избор)
	includeNotificationCount := true // bool |  (по избор)
	asTree := true // bool |  (по избор)
	maxTreeDepth := int32(56) // int32 |  (по избор)
	useFullTranslationIds := true // bool |  (по избор)
	parentId := "parentId_example" // string |  (по избор)
	searchText := "searchText_example" // string |  (по избор)
	hashTags := []string{"Inner_example"} // []string |  (по избор)
	userId := "userId_example" // string |  (по избор)
	customConfigStr := "customConfigStr_example" // string |  (по избор)
	afterCommentId := "afterCommentId_example" // string |  (по избор)
	beforeCommentId := "beforeCommentId_example" // string |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]