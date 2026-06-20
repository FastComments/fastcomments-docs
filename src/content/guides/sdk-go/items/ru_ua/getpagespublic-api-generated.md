Список страниц для тенанта. Используется настольным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` было true в разрешённой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом доступа групп запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. (необязательно)
	limit := int32(56) // int32 | 1..200, по умолчанию 50 (необязательно)
	q := "q_example" // string | Необязательный регистронезависимый фильтр по префиксу заголовка. (необязательно)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). (необязательно)
	hasComments := true // bool | Если true, возвращать только страницы с хотя бы одним комментарием. (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]