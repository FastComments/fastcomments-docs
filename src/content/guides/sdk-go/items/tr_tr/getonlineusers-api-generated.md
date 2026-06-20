Bir sayfadaki şu anda çevrimiçi izleyiciler: websocket oturumu şu anda sayfaya abone olan kişiler.
anonCount + totalCount döndürür (oda genelindeki aboneler, saymadığımız anonim izleyiciler dahil).

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName gönderin. |
| afterUserId | string | query | No | İmleç eşitleyici: önceki yanıttan nextAfterUserId gönderin. afterName ayarlandığında isim eşleşmelerinin kayıtların düşmesini önlemek için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Örnek

[inline-code-attrs-start title = 'GetOnlineUsers Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
	afterName := "afterName_example" // string | İmleç: önceki yanıttan nextAfterName gönderin. (isteğe bağlı)
	afterUserId := "afterUserId_example" // string | İmleç eşitleyici: önceki yanıttan nextAfterUserId gönderin. afterName ayarlandığında isim eşleşmelerinin kayıtların düşmesini önlemek için gereklidir. (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOnlineUsers`'ten yanıt: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]