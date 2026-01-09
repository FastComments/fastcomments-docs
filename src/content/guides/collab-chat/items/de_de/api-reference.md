### API-Übersicht

Collab Chat stellt drei REST-API-Endpunkte zum programmatischen Verwalten von Textkonversationen bereit. Mit diesen Endpunkten können Sie Annotationen abrufen, erstellen und löschen, ohne das Browser-Widget zu verwenden.

Dies sind öffentliche Endpunkte, die Benutzer über Browser-Cookies authentifizieren. Sie verwenden keine API-Schlüssel. Benutzer müssen in FastComments in ihrem Browser angemeldet sein, um auf diese Endpunkte zugreifen zu können.

### Basis-URL

Alle Collab Chat API-Endpunkte befinden sich unter:

[inline-code-attrs-start title = 'Basis-URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentifizierung

Diese Endpunkte authentifizieren Benutzer über Browser-Cookies. Sie verwenden keine API-Schlüssel. Benutzer müssen in FastComments in ihrem Browser angemeldet sein, um auf diese Endpunkte zugreifen zu können.

### Alle Konversationen abrufen

Ruft alle Textkonversationen für eine bestimmte Seite ab.

#### Endpunkt

[inline-code-attrs-start title = 'GET-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameter

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

`urlId` (Query-Parameter, erforderlich) ist die Seitenkennung, für die Sie Konversationen abrufen möchten.

#### Antwort

Die Antwort enthält den API-Status, Informationen zur aktuellen Benutzersitzung, falls authentifiziert, ein Array von Konversationen mit ihren IDs, URLs und Textranges, eine bereinigte URL-Kennung, ein Flag, das angibt, ob der aktuelle Benutzer Site-Admin oder Moderator ist, sowie WebSocket-Verbindungsdetails für die Live-Synchronisierung einschließlich `tenantIdWS`, `urlIdWS` und `userIdWS`.

#### Beispielanfrage

[inline-code-attrs-start title = 'GET-Anfragebeispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Beispielantwort

[inline-code-attrs-start title = 'GET-Antwortbeispiel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Konversation erstellen

Erstellt eine neue Textkonversation für eine bestimmte Textauswahl.

#### Endpunkt

[inline-code-attrs-start title = 'POST-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameter

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

Der Request-Body muss JSON sein und die folgenden erforderlichen Felder enthalten.

`urlId` (string, erforderlich) ist die Basis-Seitenkennung.

`urlIdWithRange` (string, erforderlich) ist die URL kombiniert mit dem Textrange, zum Beispiel `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, erforderlich) ist der Titel der Seite.

`selector` (string, erforderlich) ist der DOM-Pfad zum Element, das den ausgewählten Text enthält.

`range` (string, erforderlich) ist der serialisierte Textrange im Format `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, erforderlich) ist die ID des Kommentars, der diesen Chat initiiert hat.

`broadcastId` (string, erforderlich) ist eine UUID für die Live-Synchronisation, um Echo-Effekte zu verhindern.

#### Antwort

Die Antwort enthält den API-Status und die ID der neu erstellten Konversation.

#### Beispielanfrage

[inline-code-attrs-start title = 'POST-Anfragebeispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Beispielantwort

[inline-code-attrs-start title = 'POST-Antwortbeispiel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Konversation löschen

Löscht eine vorhandene Textkonversation. Für diesen Endpunkt sind Admin- oder Moderatorberechtigungen erforderlich, oder die Konversation muss vom authentifizierten Benutzer erstellt worden sein.

#### Endpunkt

[inline-code-attrs-start title = 'DELETE-Endpunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameter

`tenantId` (Pfadparameter, erforderlich) ist Ihre FastComments Tenant-ID.

`chatId` (Pfadparameter, erforderlich) ist die ID der zu löschenden Konversation.

`broadcastId` (Request-Body, erforderlich) ist eine UUID für die Live-Synchronisation.

#### Beispielanfrage

[inline-code-attrs-start title = 'DELETE-Anfragebeispiel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Beispielantwort

[inline-code-attrs-start title = 'DELETE-Antwortbeispiel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ratenbegrenzung

Diese Endpunkte unterliegen der standardmäßigen FastComments API-Ratenbegrenzung. Die Rate-Limit-Middleware wird pro Tenant angewendet, um Missbrauch zu verhindern und gleichzeitig normale Nutzungsmuster zu ermöglichen.

### Fehlerantworten

Alle Endpunkte geben standardmäßige HTTP-Statuscodes zurück. Eine 400-Antwort deutet auf ungültige Anfrageparameter hin. Eine 401-Antwort bedeutet, dass die Authentifizierung fehlgeschlagen ist. Eine 403-Antwort weist auf unzureichende Berechtigungen hin. Eine 404-Antwort bedeutet, dass die Konversation nicht gefunden wurde. Eine 429-Antwort zeigt an, dass das Rate-Limit überschritten wurde.

Fehlerantworten enthalten einen JSON-Body mit Details:

[inline-code-attrs-start title = 'Fehler-Antwortbeispiel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Nutzungsverfolgung

Das Erstellen von Konversationen erhöht Ihre Nutzungsmetrik `conversationCreateCount`. Alle WebSocket-Synchronisationsaktivitäten erhöhen `pubSubMessageCount` und `pubSubBandwidth`. Sie können diese Metriken in Ihrem FastComments-Dashboard unter Nutzungsanalyse überwachen.

---