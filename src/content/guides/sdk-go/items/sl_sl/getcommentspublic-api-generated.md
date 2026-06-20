zahteva
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

Vrne: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Primer

[inline-code-attrs-start title = 'Primer GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (izbirno)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (izbirno)
	sso := "sso_example" // string |  (izbirno)
	skip := int32(56) // int32 |  (izbirno)
	skipChildren := int32(56) // int32 |  (izbirno)
	limit := int32(56) // int32 |  (izbirno)
	limitChildren := int32(56) // int32 |  (izbirno)
	countChildren := true // bool |  (izbirno)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (izbirno)
	includeConfig := true // bool |  (izbirno)
	countAll := true // bool |  (izbirno)
	includei10n := true // bool |  (izbirno)
	locale := "locale_example" // string |  (izbirno)
	modules := "modules_example" // string |  (izbirno)
	isCrawler := true // bool |  (izbirno)
	includeNotificationCount := true // bool |  (izbirno)
	asTree := true // bool |  (izbirno)
	maxTreeDepth := int32(56) // int32 |  (izbirno)
	useFullTranslationIds := true // bool |  (izbirno)
	parentId := "parentId_example" // string |  (izbirno)
	searchText := "searchText_example" // string |  (izbirno)
	hashTags := []string{"Inner_example"} // []string |  (izbirno)
	userId := "userId_example" // string |  (izbirno)
	customConfigStr := "customConfigStr_example" // string |  (izbirno)
	afterCommentId := "afterCommentId_example" // string |  (izbirno)
	beforeCommentId := "beforeCommentId_example" // string |  (izbirno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]