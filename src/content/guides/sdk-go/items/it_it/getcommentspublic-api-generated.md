req
tenantId
urlId

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
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

Restituisce: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opzionale)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opzionale)
	sso := "sso_example" // string |  (opzionale)
	skip := int32(56) // int32 |  (opzionale)
	skipChildren := int32(56) // int32 |  (opzionale)
	limit := int32(56) // int32 |  (opzionale)
	limitChildren := int32(56) // int32 |  (opzionale)
	countChildren := true // bool |  (opzionale)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (opzionale)
	includeConfig := true // bool |  (opzionale)
	countAll := true // bool |  (opzionale)
	includei10n := true // bool |  (opzionale)
	locale := "locale_example" // string |  (opzionale)
	modules := "modules_example" // string |  (opzionale)
	isCrawler := true // bool |  (opzionale)
	includeNotificationCount := true // bool |  (opzionale)
	asTree := true // bool |  (opzionale)
	maxTreeDepth := int32(56) // int32 |  (opzionale)
	useFullTranslationIds := true // bool |  (opzionale)
	parentId := "parentId_example" // string |  (opzionale)
	searchText := "searchText_example" // string |  (opzionale)
	hashTags := []string{"Inner_example"} // []string |  (opzionale)
	userId := "userId_example" // string |  (opzionale)
	customConfigStr := "customConfigStr_example" // string |  (opzionale)
	afterCommentId := "afterCommentId_example" // string |  (opzionale)
	beforeCommentId := "beforeCommentId_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]