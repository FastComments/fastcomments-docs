Kiracı için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenmiş özel yapılandırmada `enableFChat` değeri true olmalıdır.
SSO gerektiren sayfalar, isteği yapan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir. |
| limit | integer | query | No | 1..200, varsayılan 50 |
| q | string | query | No | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi. |
| sortBy | string | query | No | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorumlu ilk) veya `title` (alfabetik). |
| hasComments | boolean | query | No | true ise yalnızca en az bir yorumu olan sayfaları döndürür. |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Örnek

[inline-code-attrs-start title = 'GetPagesPublic Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Önceki bir istekte `nextCursor` olarak döndürülen opak sayfalama imleci. Aynı `sortBy` ile ilişkilidir. (isteğe bağlı)
	limit := int32(56) // int32 | 1..200, varsayılan 50 (isteğe bağlı)
	q := "q_example" // string | İsteğe bağlı, büyük/küçük harfe duyarsız başlık önek filtresi. (isteğe bağlı)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Sıralama düzeni. `updatedAt` (varsayılan, en yeni ilk), `commentCount` (en çok yorumlu ilk) veya `title` (alfabetik). (isteğe bağlı)
	hasComments := true // bool | true ise yalnızca en az bir yorumu olan sayfaları döndürür. (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPagesPublic` yanıtı: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]