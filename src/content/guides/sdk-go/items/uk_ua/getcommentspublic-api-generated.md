req
tenantId
urlId

## Параметри

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| page | integer | query | Ні |  |
| direction | string | query | Ні |  |
| sso | string | query | Ні |  |
| skip | integer | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| countChildren | boolean | query | Ні |  |
| fetchPageForCommentId | string | query | Ні |  |
| includeConfig | boolean | query | Ні |  |
| countAll | boolean | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| modules | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeNotificationCount | boolean | query | Ні |  |
| asTree | boolean | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |
| parentId | string | query | Ні |  |
| searchText | string | query | Ні |  |
| hashTags | array | query | Ні |  |
| userId | string | query | Ні |  |
| customConfigStr | string | query | Ні |  |
| afterCommentId | string | query | Ні |  |
| beforeCommentId | string | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (необов'язково)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)
	skip := int32(56) // int32 |  (необов'язково)
	skipChildren := int32(56) // int32 |  (необов'язково)
	limit := int32(56) // int32 |  (необов'язково)
	limitChildren := int32(56) // int32 |  (необов'язково)
	countChildren := true // bool |  (необов'язково)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (необов'язково)
	includeConfig := true // bool |  (необов'язково)
	countAll := true // bool |  (необов'язково)
	includei10n := true // bool |  (необов'язково)
	locale := "locale_example" // string |  (необов'язково)
	modules := "modules_example" // string |  (необов'язково)
	isCrawler := true // bool |  (необов'язково)
	includeNotificationCount := true // bool |  (необов'язково)
	asTree := true // bool |  (необов'язково)
	maxTreeDepth := int32(56) // int32 |  (необов'язково)
	useFullTranslationIds := true // bool |  (необов'язково)
	parentId := "parentId_example" // string |  (необов'язково)
	searchText := "searchText_example" // string |  (необов'язково)
	hashTags := []string{"Inner_example"} // []string |  (необов'язково)
	userId := "userId_example" // string |  (необов'язково)
	customConfigStr := "customConfigStr_example" // string |  (необов'язково)
	afterCommentId := "afterCommentId_example" // string |  (необов'язково)
	beforeCommentId := "beforeCommentId_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]