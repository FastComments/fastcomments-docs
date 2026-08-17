req
tenantId
urlId

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

## Одговор

Враћа: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Пример

[inline-code-attrs-start title = 'GetCommentsPublic пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (опционално)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (опционално)
	sso := "sso_example" // string |  (опционално)
	skip := int32(56) // int32 |  (опционално)
	skipChildren := int32(56) // int32 |  (опционално)
	limit := int32(56) // int32 |  (опционално)
	limitChildren := int32(56) // int32 |  (опционално)
	countChildren := true // bool |  (опционално)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (опционално)
	includeConfig := true // bool |  (опционално)
	countAll := true // bool |  (опционално)
	includei10n := true // bool |  (опционално)
	locale := "locale_example" // string |  (опционално)
	modules := "modules_example" // string |  (опционално)
	isCrawler := true // bool |  (опционално)
	includeNotificationCount := true // bool |  (опционално)
	asTree := true // bool |  (опционално)
	maxTreeDepth := int32(56) // int32 |  (опционално)
	useFullTranslationIds := true // bool |  (опционално)
	parentId := "parentId_example" // string |  (опционално)
	searchText := "searchText_example" // string |  (опционално)
	hashTags := []string{"Inner_example"} // []string |  (опционално)
	userId := "userId_example" // string |  (опционално)
	customConfigStr := "customConfigStr_example" // string |  (опционално)
	afterCommentId := "afterCommentId_example" // string |  (опционално)
	beforeCommentId := "beforeCommentId_example" // string |  (опционално)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Грешка при позиву `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Пун HTTP одговор: %v\n", r)
	}
	// одговор од `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Одговор од `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]