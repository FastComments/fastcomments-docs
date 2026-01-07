[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Diese Route bietet die Möglichkeit, ein einzelnes autorisiertes `Vote` hinzuzufügen. Abstimmungen können `up` (+1) oder `down` (-1) sein.

[inline-code-attrs-start title = 'Vote Erstellen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymes Vote Erstellen cURL Beispiel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Erstellen Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Erstellen anonymer Abstimmungen

Anonyme Abstimmungen können erstellt werden, indem in den Abfrageparametern `anonUserId` anstelle von `userId` gesetzt wird.

Diese ID muss keinem Benutzerobjekt irgendwo entsprechen (daher anonym). Es ist einfach ein Bezeichner
für die Sitzung, damit Sie Abstimmungen in derselben Sitzung erneut abrufen können, um zu prüfen, ob für einen Kommentar
abgestimmt wurde.

Wenn Sie so etwas wie "anonyme Sitzungen" nicht haben, wie FastComments es hat, können Sie dies einfach
auf eine zufällige ID setzen, wie eine UUID (obwohl wir kleinere Bezeichner schätzen, um Platz zu sparen).

### Weitere Hinweise

- Diese API befolgt Einstellungen auf Tenant-Ebene. Wenn Sie beispielsweise das Abstimmen für eine bestimmte Seite deaktivieren und versuchen, über die API eine Abstimmung zu erstellen, schlägt dies mit dem Fehlercode `voting-disabled` fehl.
- Diese API ist standardmäßig live.
- Diese API aktualisiert die `votes` des entsprechenden `Comment`.
