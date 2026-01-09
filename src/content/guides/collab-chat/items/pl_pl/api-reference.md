### Przegląd API

Collab Chat udostępnia trzy punkty końcowe REST API do programowego zarządzania rozmowami tekstowymi. Te punkty końcowe pozwalają pobierać, tworzyć i usuwać adnotacje bez użycia widżetu w przeglądarce.

Są to publiczne punkty końcowe, które uwierzytelniają użytkowników za pomocą ciasteczek przeglądarki. Nie używają kluczy API. Użytkownicy muszą być zalogowani do FastComments w swojej przeglądarce, aby uzyskać dostęp do tych punktów końcowych.

### URL bazowy

Wszystkie punkty końcowe API Collab Chat znajdują się pod:

[inline-code-attrs-start title = 'URL bazowy'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Uwierzytelnianie

Te punkty końcowe uwierzytelniają użytkowników za pomocą ciasteczek przeglądarki. Nie używają kluczy API. Użytkownicy muszą być zalogowani do FastComments w swojej przeglądarce, aby uzyskać dostęp do tych punktów końcowych.

### Pobierz wszystkie konwersacje

Pobierz wszystkie rozmowy tekstowe dla określonej strony.

#### Punkt końcowy

[inline-code-attrs-start title = 'Punkt końcowy GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój FastComments Tenant ID.

`urlId` (parametr zapytania, wymagany) to identyfikator strony, dla której chcesz pobrać konwersacje.

#### Odpowiedź

Odpowiedź zawiera status API, informacje o bieżącej sesji użytkownika (jeśli uwierzytelniony), tablicę konwersacji z ich identyfikatorami, URL-ami i zakresami tekstu, oczyszczony identyfikator URL, flagę wskazującą, czy bieżący użytkownik jest administratorem strony lub moderatorem, oraz dane połączenia WebSocket do synchronizacji na żywo, w tym `tenantIdWS`, `urlIdWS` i `userIdWS`.

#### Przykład żądania

[inline-code-attrs-start title = 'Przykład żądania GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Przykład odpowiedzi

[inline-code-attrs-start title = 'Przykład odpowiedzi GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Utwórz konwersację

Utwórz nową konwersację tekstową dla określonego zaznaczenia tekstu.

#### Punkt końcowy

[inline-code-attrs-start title = 'Punkt końcowy POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój FastComments Tenant ID.

Ciało żądania musi być w formacie JSON i zawierać następujące wymagane pola.

`urlId` (string, wymagany) to podstawowy identyfikator strony.

`urlIdWithRange` (string, wymagany) to URL połączony z zakresem tekstu, na przykład `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, wymagany) to tytuł strony.

`selector` (string, wymagany) to ścieżka DOM do elementu zawierającego zaznaczony tekst.

`range` (string, wymagany) to zserializowany zakres tekstu w formacie `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, wymagany) to ID komentarza, który zainicjował tę rozmowę.

`broadcastId` (string, wymagany) to UUID do synchronizacji na żywo, zapobiegający efektom echa.

#### Odpowiedź

Odpowiedź zawiera status API oraz identyfikator nowo utworzonej konwersacji.

#### Przykład żądania

[inline-code-attrs-start title = 'Przykład żądania POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Przykład odpowiedzi

[inline-code-attrs-start title = 'Przykład odpowiedzi POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Usuń konwersację

Usuń istniejącą konwersację tekstową. Ten punkt końcowy wymaga uprawnień administratora lub moderatora, albo konwersacja musi być utworzona przez uwierzytelnionego użytkownika.

#### Punkt końcowy

[inline-code-attrs-start title = 'Punkt końcowy DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój FastComments Tenant ID.

`chatId` (parametr ścieżki, wymagany) to ID konwersacji do usunięcia.

`broadcastId` (ciało żądania, wymagany) to UUID do synchronizacji na żywo.

#### Przykład żądania

[inline-code-attrs-start title = 'Przykład żądania DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Przykład odpowiedzi

[inline-code-attrs-start title = 'Przykład odpowiedzi DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ograniczanie liczby żądań

Te punkty końcowe podlegają standardowym ograniczeniom liczby żądań API FastComments. Middleware limitujący żądania działa na poziomie najemcy (tenant), aby zapobiegać nadużyciom przy jednoczesnym umożliwieniu normalnych wzorców użycia.

### Odpowiedzi z błędami

Wszystkie punkty końcowe zwracają standardowe kody statusu HTTP. Odpowiedź 400 oznacza nieprawidłowe parametry żądania. Odpowiedź 401 oznacza niepowodzenie uwierzytelnienia. Odpowiedź 403 wskazuje na niewystarczające uprawnienia. Odpowiedź 404 oznacza, że konwersacja nie została znaleziona. Odpowiedź 429 oznacza przekroczenie limitu żądań.

Odpowiedzi z błędami zawierają ciało JSON z szczegółami:

[inline-code-attrs-start title = 'Przykład odpowiedzi z błędem'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Śledzenie użycia

Tworzenie konwersacji zwiększa Twój miernik wykorzystania `conversationCreateCount`. Cała aktywność synchronizacji przez WebSocket zwiększa `pubSubMessageCount` i `pubSubBandwidth`. Możesz monitorować te metryki w panelu FastComments w sekcji analityki użycia.