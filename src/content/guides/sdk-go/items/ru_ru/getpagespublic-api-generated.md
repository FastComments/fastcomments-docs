Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` был установлен в true в разрешённой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом прав доступа групп запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим количеством комментариев), или `title` (в алфавитном порядке). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы с хотя бы одним комментарием. |

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
	cursor := "cursor_example" // string | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. (опционально)
	limit := int32(56) // int32 | 1..200, по умолчанию 50 (опционально)
	q := "q_example" // string | Необязательный регистронезависимый фильтр по префиксу заголовка. (опционально)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим количеством комментариев), или `title` (в алфавитном порядке). (опционально)
	hasComments := true // bool | Если true, возвращать только страницы с хотя бы одним комментарием. (опционально)

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