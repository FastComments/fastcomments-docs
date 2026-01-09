req
tenantId
urlId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| page | integer | query | Nie |  |
| direction | string | query | Nie |  |
| sso | string | query | Nie |  |
| skip | integer | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| countChildren | boolean | query | Nie |  |
| fetchPageForCommentId | string | query | Nie |  |
| includeConfig | boolean | query | Nie |  |
| countAll | boolean | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| modules | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeNotificationCount | boolean | query | Nie |  |
| asTree | boolean | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |
| parentId | string | query | Nie |  |
| searchText | string | query | Nie |  |
| hashTags | array | query | Nie |  |
| userId | string | query | Nie |  |
| customConfigStr | string | query | Nie |  |
| afterCommentId | string | query | Nie |  |
| beforeCommentId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opcjonalne)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)
	skip := int32(56) // int32 |  (opcjonalne)
	skipChildren := int32(56) // int32 |  (opcjonalne)
	limit := int32(56) // int32 |  (opcjonalne)
	limitChildren := int32(56) // int32 |  (opcjonalne)
	countChildren := true // bool |  (opcjonalne)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (opcjonalne)
	includeConfig := true // bool |  (opcjonalne)
	countAll := true // bool |  (opcjonalne)
	includei10n := true // bool |  (opcjonalne)
	locale := "locale_example" // string |  (opcjonalne)
	modules := "modules_example" // string |  (opcjonalne)
	isCrawler := true // bool |  (opcjonalne)
	includeNotificationCount := true // bool |  (opcjonalne)
	asTree := true // bool |  (opcjonalne)
	maxTreeDepth := int32(56) // int32 |  (opcjonalne)
	useFullTranslationIds := true // bool |  (opcjonalne)
	parentId := "parentId_example" // string |  (opcjonalne)
	searchText := "searchText_example" // string |  (opcjonalne)
	hashTags := []string{"Inner_example"} // []string |  (opcjonalne)
	userId := "userId_example" // string |  (opcjonalne)
	customConfigStr := "customConfigStr_example" // string |  (opcjonalne)
	afterCommentId := "afterCommentId_example" // string |  (opcjonalne)
	beforeCommentId := "beforeCommentId_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]