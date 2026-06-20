## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| page | integer | query | Não |  |
| limit | integer | query | Não |  |
| skip | integer | query | Não |  |
| asTree | boolean | query | Não |  |
| skipChildren | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |
| contextUserId | string | query | Não |  |
| hashTag | string | query | Não |  |
| parentId | string | query | Não |  |
| direction | string | query | Não |  |
| fromDate | integer | query | Não |  |
| toDate | integer | query | Não |  |

## Resposta

Retorna: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_get_comments_response.go)

## Exemplo

[inline-code-attrs-start title = 'Exemplo GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opcional)
	limit := int32(56) // int32 |  (opcional)
	skip := int32(56) // int32 |  (opcional)
	asTree := true // bool |  (opcional)
	skipChildren := int32(56) // int32 |  (opcional)
	limitChildren := int32(56) // int32 |  (opcional)
	maxTreeDepth := int32(56) // int32 |  (opcional)
	urlId := "urlId_example" // string |  (opcional)
	userId := "userId_example" // string |  (opcional)
	anonUserId := "anonUserId_example" // string |  (opcional)
	contextUserId := "contextUserId_example" // string |  (opcional)
	hashTag := "hashTag_example" // string |  (opcional)
	parentId := "parentId_example" // string |  (opcional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcional)
	fromDate := int64(789) // int64 |  (opcional)
	toDate := int64(789) // int64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).FromDate(fromDate).ToDate(toDate).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// resposta de `GetComments`: APIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]