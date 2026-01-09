req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| page | integer | query | Ne |  |
| direction | string | query | Ne |  |
| sso | string | query | Ne |  |
| skip | integer | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| countChildren | boolean | query | Ne |  |
| fetchPageForCommentId | string | query | Ne |  |
| includeConfig | boolean | query | Ne |  |
| countAll | boolean | query | Ne |  |
| includei10n | boolean | query | Ne |  |
| locale | string | query | Ne |  |
| modules | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |
| includeNotificationCount | boolean | query | Ne |  |
| asTree | boolean | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| useFullTranslationIds | boolean | query | Ne |  |
| parentId | string | query | Ne |  |
| searchText | string | query | Ne |  |
| hashTags | array | query | Ne |  |
| userId | string | query | Ne |  |
| customConfigStr | string | query | Ne |  |
| afterCommentId | string | query | Ne |  |
| beforeCommentId | string | query | Ne |  |

## Odgovor

Vrača: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (neobvezno)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (neobvezno)
	sso := "sso_example" // string |  (neobvezno)
	skip := int32(56) // int32 |  (neobvezno)
	skipChildren := int32(56) // int32 |  (neobvezno)
	limit := int32(56) // int32 |  (neobvezno)
	limitChildren := int32(56) // int32 |  (neobvezno)
	countChildren := true // bool |  (neobvezno)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (neobvezno)
	includeConfig := true // bool |  (neobvezno)
	countAll := true // bool |  (neobvezno)
	includei10n := true // bool |  (neobvezno)
	locale := "locale_example" // string |  (neobvezno)
	modules := "modules_example" // string |  (neobvezno)
	isCrawler := true // bool |  (neobvezno)
	includeNotificationCount := true // bool |  (neobvezno)
	asTree := true // bool |  (neobvezno)
	maxTreeDepth := int32(56) // int32 |  (neobvezno)
	useFullTranslationIds := true // bool |  (neobvezno)
	parentId := "parentId_example" // string |  (neobvezno)
	searchText := "searchText_example" // string |  (neobvezno)
	hashTags := []string{"Inner_example"} // []string |  (neobvezno)
	userId := "userId_example" // string |  (neobvezno)
	customConfigStr := "customConfigStr_example" // string |  (neobvezno)
	afterCommentId := "afterCommentId_example" // string |  (neobvezno)
	beforeCommentId := "beforeCommentId_example" // string |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]

---