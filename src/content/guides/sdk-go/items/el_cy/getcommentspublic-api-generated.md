req
tenantId
urlId

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| direction | string | query | Όχι |  |
| sso | string | query | Όχι |  |
| skip | integer | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| countChildren | boolean | query | Όχι |  |
| fetchPageForCommentId | string | query | Όχι |  |
| includeConfig | boolean | query | Όχι |  |
| countAll | boolean | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| modules | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |
| includeNotificationCount | boolean | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| useFullTranslationIds | boolean | query | Όχι |  |
| parentId | string | query | Όχι |  |
| searchText | string | query | Όχι |  |
| hashTags | array | query | Όχι |  |
| userId | string | query | Όχι |  |
| customConfigStr | string | query | Όχι |  |
| afterCommentId | string | query | Όχι |  |
| beforeCommentId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (προαιρετικό)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (προαιρετικό)
	sso := "sso_example" // string |  (προαιρετικό)
	skip := int32(56) // int32 |  (προαιρετικό)
	skipChildren := int32(56) // int32 |  (προαιρετικό)
	limit := int32(56) // int32 |  (προαιρετικό)
	limitChildren := int32(56) // int32 |  (προαιρετικό)
	countChildren := true // bool |  (προαιρετικό)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (προαιρετικό)
	includeConfig := true // bool |  (προαιρετικό)
	countAll := true // bool |  (προαιρετικό)
	includei10n := true // bool |  (προαιρετικό)
	locale := "locale_example" // string |  (προαιρετικό)
	modules := "modules_example" // string |  (προαιρετικό)
	isCrawler := true // bool |  (προαιρετικό)
	includeNotificationCount := true // bool |  (προαιρετικό)
	asTree := true // bool |  (προαιρετικό)
	maxTreeDepth := int32(56) // int32 |  (προαιρετικό)
	useFullTranslationIds := true // bool |  (προαιρετικό)
	parentId := "parentId_example" // string |  (προαιρετικό)
	searchText := "searchText_example" // string |  (προαιρετικό)
	hashTags := []string{"Inner_example"} // []string |  (προαιρετικό)
	userId := "userId_example" // string |  (προαιρετικό)
	customConfigStr := "customConfigStr_example" // string |  (προαιρετικό)
	afterCommentId := "afterCommentId_example" // string |  (προαιρετικό)
	beforeCommentId := "beforeCommentId_example" // string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]