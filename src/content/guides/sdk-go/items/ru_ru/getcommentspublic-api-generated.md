req
tenantId
urlId

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Нет |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |
| skip | integer | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| countChildren | boolean | query | Нет |  |
| fetchPageForCommentId | string | query | Нет |  |
| includeConfig | boolean | query | Нет |  |
| countAll | boolean | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| modules | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeNotificationCount | boolean | query | Нет |  |
| asTree | boolean | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |
| parentId | string | query | Нет |  |
| searchText | string | query | Нет |  |
| hashTags | array | query | Нет |  |
| userId | string | query | Нет |  |
| customConfigStr | string | query | Нет |  |
| afterCommentId | string | query | Нет |  |
| beforeCommentId | string | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// ответ от `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]