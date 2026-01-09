req
tenantId
urlId

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (незадължително)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (незадължително)
	sso := "sso_example" // string |  (незадължително)
	skip := int32(56) // int32 |  (незадължително)
	skipChildren := int32(56) // int32 |  (незадължително)
	limit := int32(56) // int32 |  (незадължително)
	limitChildren := int32(56) // int32 |  (незадължително)
	countChildren := true // bool |  (незадължително)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (незадължително)
	includeConfig := true // bool |  (незадължително)
	countAll := true // bool |  (незадължително)
	includei10n := true // bool |  (незадължително)
	locale := "locale_example" // string |  (незадължително)
	modules := "modules_example" // string |  (незадължително)
	isCrawler := true // bool |  (незадължително)
	includeNotificationCount := true // bool |  (незадължително)
	asTree := true // bool |  (незадължително)
	maxTreeDepth := int32(56) // int32 |  (незадължително)
	useFullTranslationIds := true // bool |  (незадължително)
	parentId := "parentId_example" // string |  (незадължително)
	searchText := "searchText_example" // string |  (незадължително)
	hashTags := []string{"Inner_example"} // []string |  (незадължително)
	userId := "userId_example" // string |  (незадължително)
	customConfigStr := "customConfigStr_example" // string |  (незадължително)
	afterCommentId := "afterCommentId_example" // string |  (незадължително)
	beforeCommentId := "beforeCommentId_example" // string |  (незадължително)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]