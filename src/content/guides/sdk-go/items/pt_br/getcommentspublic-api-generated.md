req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| page | integer | query | Não |  |
| direction | string | query | Não |  |
| sso | string | query | Não |  |
| skip | integer | query | Não |  |
| skipChildren | integer | query | Não |  |
| limit | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| countChildren | boolean | query | Não |  |
| fetchPageForCommentId | string | query | Não |  |
| includeConfig | boolean | query | Não |  |
| countAll | boolean | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| modules | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeNotificationCount | boolean | query | Não |  |
| asTree | boolean | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |
| parentId | string | query | Não |  |
| searchText | string | query | Não |  |
| hashTags | array | query | Não |  |
| userId | string | query | Não |  |
| customConfigStr | string | query | Não |  |
| afterCommentId | string | query | Não |  |
| beforeCommentId | string | query | Não |  |

## Resposta

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opcional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcional)
	sso := "sso_example" // string |  (opcional)
	skip := int32(56) // int32 |  (opcional)
	skipChildren := int32(56) // int32 |  (opcional)
	limit := int32(56) // int32 |  (opcional)
	limitChildren := int32(56) // int32 |  (opcional)
	countChildren := true // bool |  (opcional)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (opcional)
	includeConfig := true // bool |  (opcional)
	countAll := true // bool |  (opcional)
	includei10n := true // bool |  (opcional)
	locale := "locale_example" // string |  (opcional)
	modules := "modules_example" // string |  (opcional)
	isCrawler := true // bool |  (opcional)
	includeNotificationCount := true // bool |  (opcional)
	asTree := true // bool |  (opcional)
	maxTreeDepth := int32(56) // int32 |  (opcional)
	useFullTranslationIds := true // bool |  (opcional)
	parentId := "parentId_example" // string |  (opcional)
	searchText := "searchText_example" // string |  (opcional)
	hashTags := []string{"Inner_example"} // []string |  (opcional)
	userId := "userId_example" // string |  (opcional)
	customConfigStr := "customConfigStr_example" // string |  (opcional)
	afterCommentId := "afterCommentId_example" // string |  (opcional)
	beforeCommentId := "beforeCommentId_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]