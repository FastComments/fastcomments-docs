---
### API-Übersicht

Image Chat stellt drei REST-API-Endpunkte bereit, um Bild-Konversationen programmgesteuert zu verwalten. Über diese Endpunkte können Sie Marker abrufen, erstellen und löschen, ohne das Browser-Widget zu verwenden.

Alle API-Endpunkte erfordern eine Authentifizierung und folgen den Standardmustern der FastComments-API. Dies sind öffentliche Endpunkte, die sich über Browser-Cookies authentifizieren und nicht über API-Schlüssel.

### Basis-URL

Alle Image-Chat-API-Endpunkte befinden sich unter:

```
https://fastcomments.com/comment-image-chats
```

### Authentifizierung

Diese Endpunkte authentifizieren Benutzer über Browser-Cookies. Sie verwenden keine API-Schlüssel. Benutzer müssen in ihrem Browser bei FastComments angemeldet sein, um auf diese Endpunkte zugreifen zu können.

### Alle Konversationen abrufen

Alle Bild-Konversationen für ein bestimmtes Bild abrufen.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

`urlId` (Query-Parameter, erforderlich) ist die Bildkennung, für die Sie Konversationen abrufen möchten.

#### Response

Die Antwort enthält den API-Status, Informationen zur aktuellen Benutzersitzung (falls authentifiziert), ein Array von Konversationen mit ihren IDs, URLs und X/Y-Koordinaten, einen bereinigten URL-Identifier, ein Flag, das angibt, ob der aktuelle Benutzer Site-Admin oder Moderator ist, sowie WebSocket-Verbindungsdetails für die Live-Synchronisation einschließlich `tenantIdWS`, `urlIdWS` und `userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

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

### Konversation erstellen

Erstellen Sie eine neue Bild-Konversation für eine bestimmte Position auf einem Bild.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

Der Request-Body muss JSON sein und die folgenden erforderlichen Felder enthalten.

`urlId` (string, erforderlich) ist die Basisseitenkennung.

`windowUrlId` (string, erforderlich) ist die URL kombiniert mit der Bildquelle und den Koordinaten, zum Beispiel `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, erforderlich) ist der Titel der Seite.

`src` (string, erforderlich) ist die Bildquelle (URL).

`x` (number, erforderlich) ist die X-Koordinate als Prozentsatz (0–100).

`y` (number, erforderlich) ist die Y-Koordinate als Prozentsatz (0–100).

`createdFromCommentId` (string, erforderlich) ist die ID des Kommentars, der diesen Chat initiiert hat.

`broadcastId` (string, erforderlich) ist eine UUID für die Live-Synchronisation, um Echo-Effekte zu verhindern.

#### Response

Die Antwort enthält den API-Status und die ID der neu erstellten Konversation.

#### Example Request

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

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Konversation löschen

Löscht eine vorhandene Bild-Konversation. Dieser Endpunkt erfordert Admin- oder Moderatorrechte, oder die Konversation muss vom authentifizierten Benutzer erstellt worden sein.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

`chatId` (Pfadparameter, erforderlich) ist die ID der zu löschenden Konversation.

`broadcastId` (Request-Body, erforderlich) ist eine UUID für die Live-Synchronisation.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Koordinatensystem

Die X- und Y-Koordinaten werden als Prozentsätze der Bilddimensionen gespeichert. X repräsentiert die horizontale Position vom linken Rand (0 = linker Rand, 100 = rechter Rand). Y repräsentiert die vertikale Position vom oberen Rand (0 = oberer Rand, 100 = unterer Rand).

Diese Prozentwerte können Dezimalstellen für Genauigkeit enthalten. Zum Beispiel bedeutet x: 25.5, dass es 25,5 % vom linken Rand des Bildes entfernt ist.

### Ratenbegrenzung

Diese Endpunkte unterliegen der standardmäßigen FastComments-API-Ratenbegrenzung. Die Rate-Limit-Middleware wird pro Tenant angewendet, um Missbrauch zu verhindern und gleichzeitig normale Nutzungsmuster zuzulassen.

### Fehlerantworten

Alle Endpunkte geben standardmäßige HTTP-Statuscodes zurück. Eine 400-Antwort weist auf ungültige Anfrageparameter hin. Eine 401-Antwort bedeutet, dass die Authentifizierung fehlgeschlagen ist. Eine 403-Antwort zeigt unzureichende Berechtigungen an. Eine 404-Antwort bedeutet, dass die Konversation nicht gefunden wurde. Eine 429-Antwort zeigt an, dass das Rate-Limit überschritten wurde.

Fehlerantworten enthalten einen JSON-Body mit Details:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Nutzungsverfolgung

Das Erstellen von Konversationen erhöht Ihre Nutzungsmetrik `conversationCreateCount`. Alle WebSocket-Synchronisationsaktivitäten erhöhen `pubSubMessageCount` und `pubSubBandwidth`. Sie können diese Metriken in Ihrem FastComments-Dashboard unter den Nutzungs-Analytics überwachen.

---