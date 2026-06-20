req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Renvoie : [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optionnel)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optionnel)
	sso := "sso_example" // string |  (optionnel)
	skip := int32(56) // int32 |  (optionnel)
	skipChildren := int32(56) // int32 |  (optionnel)
	limit := int32(56) // int32 |  (optionnel)
	limitChildren := int32(56) // int32 |  (optionnel)
	countChildren := true // bool |  (optionnel)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (optionnel)
	includeConfig := true // bool |  (optionnel)
	countAll := true // bool |  (optionnel)
	includei10n := true // bool |  (optionnel)
	locale := "locale_example" // string |  (optionnel)
	modules := "modules_example" // string |  (optionnel)
	isCrawler := true // bool |  (optionnel)
	includeNotificationCount := true // bool |  (optionnel)
	asTree := true // bool |  (optionnel)
	maxTreeDepth := int32(56) // int32 |  (optionnel)
	useFullTranslationIds := true // bool |  (optionnel)
	parentId := "parentId_example" // string |  (optionnel)
	searchText := "searchText_example" // string |  (optionnel)
	hashTags := []string{"Inner_example"} // []string |  (optionnel)
	userId := "userId_example" // string |  (optionnel)
	customConfigStr := "customConfigStr_example" // string |  (optionnel)
	afterCommentId := "afterCommentId_example" // string |  (optionnel)
	beforeCommentId := "beforeCommentId_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]