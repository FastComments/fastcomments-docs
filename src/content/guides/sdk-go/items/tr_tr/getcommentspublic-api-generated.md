req
tenantId
urlId

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
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

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetCommentsPublic Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// `GetCommentsPublic`'ten gelen yanıt: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]