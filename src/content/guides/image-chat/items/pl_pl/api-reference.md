### Przegląd API

Image Chat udostępnia trzy końcówki REST API do zarządzania rozmowami obrazkowymi programowo. Te końcówki pozwalają pobierać, tworzyć i usuwać markery bez użycia widżetu w przeglądarce.

Wszystkie końcówki API wymagają uwierzytelnienia i stosują standardowe wzorce FastComments API. Są to publiczne końcówki, które uwierzytelniają za pomocą ciasteczek przeglądarki, a nie kluczy API.

### URL bazowy

Wszystkie końcówki API Image Chat znajdują się pod:

```
https://fastcomments.com/comment-image-chats
```

### Uwierzytelnianie

Te końcówki uwierzytelniają użytkowników za pomocą ciasteczek przeglądarki. Nie używają kluczy API. Użytkownicy muszą być zalogowani w FastComments w swojej przeglądarce, aby uzyskać dostęp do tych końcówek.

### Pobierz wszystkie rozmowy

Pobierz wszystkie rozmowy dotyczące konkretnego obrazu.

#### Punkt końcowy

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój identyfikator najemcy (Tenant ID) FastComments.

`urlId` (parametr zapytania, wymagany) to identyfikator obrazu, dla którego chcesz pobrać rozmowy.

#### Odpowiedź

Odpowiedź zawiera status API, informacje o bieżącej sesji użytkownika, jeśli jest uwierzytelniony, tablicę rozmów z ich identyfikatorami, URL-ami oraz współrzędnymi X/Y, oczyszczony identyfikator URL, flagę wskazującą, czy bieżący użytkownik jest administratorem strony lub moderatorem, oraz szczegóły połączenia WebSocket do synchronizacji na żywo, w tym `tenantIdWS`, `urlIdWS` i `userIdWS`.

#### Przykładowe żądanie

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Przykładowa odpowiedź

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Utwórz rozmowę

Utwórz nową rozmowę obrazkową dla konkretnego miejsca na obrazie.

#### Punkt końcowy

```
POST /comment-image-chats/:tenantId
```

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój identyfikator najemcy FastComments.

Ciało żądania musi być w formacie JSON i zawierać następujące wymagane pola.

`urlId` (string, wymagany) to podstawowy identyfikator strony.

`windowUrlId` (string, wymagany) to URL połączony ze źródłem obrazu i współrzędnymi, na przykład `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, wymagany) to tytuł strony.

`src` (string, wymagany) to URL źródła obrazu.

`x` (number, wymagany) to współrzędna X jako procent (0-100).

`y` (number, wymagany) to współrzędna Y jako procent (0-100).

`createdFromCommentId` (string, wymagany) to identyfikator komentarza, który zainicjował tę rozmowę.

`broadcastId` (string, wymagany) to UUID do synchronizacji na żywo, zapobiegający efektom echa.

#### Odpowiedź

Odpowiedź zawiera status API oraz identyfikator nowo utworzonej rozmowy.

#### Przykładowe żądanie

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Przykładowa odpowiedź

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Usuń rozmowę

Usuń istniejącą rozmowę obrazkową. Ta końcówka wymaga uprawnień administratora lub moderatora, albo rozmowa musi być utworzona przez uwierzytelnionego użytkownika.

#### Punkt końcowy

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametry

`tenantId` (parametr ścieżki, wymagany) to Twój identyfikator najemcy FastComments.

`chatId` (parametr ścieżki, wymagany) to identyfikator rozmowy, którą chcesz usunąć.

`broadcastId` (ciało żądania, wymagane) to UUID do synchronizacji na żywo.

#### Przykładowe żądanie

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Przykładowa odpowiedź

```json
{
  "status": "success"
}
```

### Układ współrzędnych

Współrzędne X i Y są przechowywane jako procenty wymiarów obrazu. X reprezentuje pozycję poziomą od lewej krawędzi (0 = lewa krawędź, 100 = prawa krawędź). Y reprezentuje pozycję pionową od górnej krawędzi (0 = górna krawędź, 100 = dolna krawędź).

Wartości procentowe mogą zawierać miejsca dziesiętne dla precyzji. Na przykład, x: 25.5 oznacza 25.5% od lewej krawędzi obrazu.

### Ograniczenia szybkości (Rate Limiting)

Te końcówki podlegają standardowym ograniczeniom szybkości FastComments API. Middleware ograniczający liczbę żądań działa per-tenant, aby zapobiegać nadużyciom przy jednoczesnym umożliwieniu normalnych wzorców użycia.

### Odpowiedzi błędów

Wszystkie końcówki zwracają standardowe kody statusu HTTP. Odpowiedź 400 oznacza nieprawidłowe parametry żądania. Odpowiedź 401 oznacza nieudane uwierzytelnienie. Odpowiedź 403 wskazuje na niewystarczające uprawnienia. Odpowiedź 404 oznacza, że rozmowa nie została znaleziona. Odpowiedź 429 oznacza przekroczenie limitu żądań.

Odpowiedzi błędów zawierają ciało JSON z szczegółami:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Śledzenie użycia

Tworzenie rozmów zwiększa Twoją metrykę użycia `conversationCreateCount`. Cała aktywność synchronizacji WebSocket zwiększa `pubSubMessageCount` oraz `pubSubBandwidth`. Możesz monitorować te metryki w panelu FastComments w zakładce analytics dotyczącej użycia.