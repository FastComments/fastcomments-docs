[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia tworzenie komentarzy.

Typowe przypadki użycia to niestandardowe interfejsy użytkownika, integracje lub importy.

Uwagi:

- Ten interfejs API może zaktualizować widżet komentarzy "na żywo", jeśli jest taka potrzeba (zwiększa to `creditsCost` z `1` do `2`).
- Ten interfejs API automatycznie utworzy obiekty użytkowników w naszym systemie, jeśli podany zostanie adres e-mail.
- Próba zapisania dwóch komentarzy z różnymi adresami e-mail, ale taką samą nazwą użytkownika, spowoduje błąd dla drugiego komentarza. 
- Jeśli określasz `parentId`, a komentarz potomny ma `notificationSentForParent` ustawione na false, **wyślemy powiadomienia dla komentarza nadrzędnego**. Odbywa się to co godzinę (grupujemy powiadomienia, aby zmniejszyć liczbę wysyłanych e-maili).
- Jeśli chcesz wysyłać e-maile powitalne podczas tworzenia użytkowników lub e-maile weryfikacyjne komentarzy, ustaw `sendEmails` na `true` w parametrach zapytania.
- Komentarze utworzone za pomocą tego interfejsu API będą widoczne na stronach Analityka i Moderacja w aplikacji administracyjnej.
- „wulgaryzmy” nadal są maskowane w nazwach komentujących i tekście komentarza, jeśli ustawienie jest włączone.
- Komentarze utworzone przez ten interfejs API nadal mogą być sprawdzane pod kątem spamu, jeśli zajdzie taka potrzeba.
- Konfiguracje, takie jak maksymalna długość komentarza, jeśli są skonfigurowane za pomocą strony administracyjnej Customization Rule, będą miały tu zastosowanie.

Minimalne dane wymagane do przesłania, które będą wyświetlane w widżecie komentarzy, są następujące:

[inline-code-attrs-start title = 'Minimalny przykład POST komentarza cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Bardziej realistyczne żądanie może wyglądać następująco:

[inline-code-attrs-start title = 'Przykład POST komentarza cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania POST komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Czy komentarz powinien pojawić się "na żywo" dla użytkowników przeglądających instancje widżetu komentarzy o tym samym urlId. Uwaga: Podwaja koszt kredytów z 1 do 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi POST komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    /** Utworzony komentarz. **/
    comment?: Comment
    /** Powiązany użytkownik, który mógł już istnieć lub nie. **/
    user?: User
}
[inline-code-end]