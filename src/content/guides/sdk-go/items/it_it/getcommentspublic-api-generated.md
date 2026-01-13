req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
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

## Risposta

Restituisce: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (facoltativo)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (facoltativo)
	sso := "sso_example" // string |  (facoltativo)
	skip := int32(56) // int32 |  (facoltativo)
	skipChildren := int32(56) // int32 |  (facoltativo)
	limit := int32(56) // int32 |  (facoltativo)
	limitChildren := int32(56) // int32 |  (facoltativo)
	countChildren := true // bool |  (facoltativo)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (facoltativo)
	includeConfig := true // bool |  (facoltativo)
	countAll := true // bool |  (facoltativo)
	includei10n := true // bool |  (facoltativo)
	locale := "locale_example" // string |  (facoltativo)
	modules := "modules_example" // string |  (facoltativo)
	isCrawler := true // bool |  (facoltativo)
	includeNotificationCount := true // bool |  (facoltativo)
	asTree := true // bool |  (facoltativo)
	maxTreeDepth := int32(56) // int32 |  (facoltativo)
	useFullTranslationIds := true // bool |  (facoltativo)
	parentId := "parentId_example" // string |  (facoltativo)
	searchText := "searchText_example" // string |  (facoltativo)
	hashTags := []string{"Inner_example"} // []string |  (facoltativo)
	userId := "userId_example" // string |  (facoltativo)
	customConfigStr := "customConfigStr_example" // string |  (facoltativo)
	afterCommentId := "afterCommentId_example" // string |  (facoltativo)
	beforeCommentId := "beforeCommentId_example" // string |  (facoltativo)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]