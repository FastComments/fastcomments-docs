erforderlich
tenantId
urlId

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nein |  |
| direction | string | query | Nein |  |
| sso | string | query | Nein |  |
| skip | integer | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| countChildren | boolean | query | Nein |  |
| fetchPageForCommentId | string | query | Nein |  |
| includeConfig | boolean | query | Nein |  |
| countAll | boolean | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| modules | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |
| includeNotificationCount | boolean | query | Nein |  |
| asTree | boolean | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| useFullTranslationIds | boolean | query | Nein |  |
| parentId | string | query | Nein |  |
| searchText | string | query | Nein |  |
| hashTags | array | query | Nein |  |
| userId | string | query | Nein |  |
| customConfigStr | string | query | Nein |  |
| afterCommentId | string | query | Nein |  |
| beforeCommentId | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetCommentsPublic Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optional)
	sso := "sso_example" // string |  (optional)
	skip := int32(56) // int32 |  (optional)
	skipChildren := int32(56) // int32 |  (optional)
	limit := int32(56) // int32 |  (optional)
	limitChildren := int32(56) // int32 |  (optional)
	countChildren := true // bool |  (optional)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (optional)
	includeConfig := true // bool |  (optional)
	countAll := true // bool |  (optional)
	includei10n := true // bool |  (optional)
	locale := "locale_example" // string |  (optional)
	modules := "modules_example" // string |  (optional)
	isCrawler := true // bool |  (optional)
	includeNotificationCount := true // bool |  (optional)
	asTree := true // bool |  (optional)
	maxTreeDepth := int32(56) // int32 |  (optional)
	useFullTranslationIds := true // bool |  (optional)
	parentId := "parentId_example" // string |  (optional)
	searchText := "searchText_example" // string |  (optional)
	hashTags := []string{"Inner_example"} // []string |  (optional)
	userId := "userId_example" // string |  (optional)
	customConfigStr := "customConfigStr_example" // string |  (optional)
	afterCommentId := "afterCommentId_example" // string |  (optional)
	beforeCommentId := "beforeCommentId_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]