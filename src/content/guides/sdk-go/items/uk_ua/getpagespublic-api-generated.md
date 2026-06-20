Перелік сторінок для орендаря. Використовується десктоп-клієнтом FChat для заповнення списку його кімнат.
Потрібно, щоб `enableFChat` був true у визначеній кастомній конфігурації для кожної сторінки.
Сторінки, що потребують SSO, фільтруються згідно з груповим доступом користувача, який виконує запит.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, який повертається як `nextCursor` у попередньому запиті. Пов'язаний із тим самим `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий фільтр префіксу заголовка без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (за абеткою). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Непрозорий курсор пагінації, який повертається як `nextCursor` у попередньому запиті. Пов'язаний із тим самим `sortBy`. (необов'язково)
	limit := int32(56) // int32 | 1..200, за замовчуванням 50 (необов'язково)
	q := "q_example" // string | Необов'язковий фільтр префіксу заголовка без урахування регістру. (необов'язково)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (за абеткою). (необов'язково)
	hasComments := true // bool | Якщо true, повертати лише сторінки з принаймні одним коментарем. (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---