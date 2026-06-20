req
tenantId
urlId

## Parametreler

| Name | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| direction | string | query | Hayır |  |
| sso | string | query | Hayır |  |
| skip | integer | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| countChildren | boolean | query | Hayır |  |
| fetchPageForCommentId | string | query | Hayır |  |
| includeConfig | boolean | query | Hayır |  |
| countAll | boolean | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| modules | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeNotificationCount | boolean | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |
| parentId | string | query | Hayır |  |
| searchText | string | query | Hayır |  |
| hashTags | array | query | Hayır |  |
| userId | string | query | Hayır |  |
| customConfigStr | string | query | Hayır |  |
| afterCommentId | string | query | Hayır |  |
| beforeCommentId | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## Örnek

[inline-code-attrs-start title = 'GetCommentsPublic Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (isteğe bağlı)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)
	skip := int32(56) // int32 |  (isteğe bağlı)
	skipChildren := int32(56) // int32 |  (isteğe bağlı)
	limit := int32(56) // int32 |  (isteğe bağlı)
	limitChildren := int32(56) // int32 |  (isteğe bağlı)
	countChildren := true // bool |  (isteğe bağlı)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (isteğe bağlı)
	includeConfig := true // bool |  (isteğe bağlı)
	countAll := true // bool |  (isteğe bağlı)
	includei10n := true // bool |  (isteğe bağlı)
	locale := "locale_example" // string |  (isteğe bağlı)
	modules := "modules_example" // string |  (isteğe bağlı)
	isCrawler := true // bool |  (isteğe bağlı)
	includeNotificationCount := true // bool |  (isteğe bağlı)
	asTree := true // bool |  (isteğe bağlı)
	maxTreeDepth := int32(56) // int32 |  (isteğe bağlı)
	useFullTranslationIds := true // bool |  (isteğe bağlı)
	parentId := "parentId_example" // string |  (isteğe bağlı)
	searchText := "searchText_example" // string |  (isteğe bağlı)
	hashTags := []string{"Inner_example"} // []string |  (isteğe bağlı)
	userId := "userId_example" // string |  (isteğe bağlı)
	customConfigStr := "customConfigStr_example" // string |  (isteğe bağlı)
	afterCommentId := "afterCommentId_example" // string |  (isteğe bağlı)
	beforeCommentId := "beforeCommentId_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsPublic`'tan yanıt: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]