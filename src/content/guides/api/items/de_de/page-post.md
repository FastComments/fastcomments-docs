[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, Seiten zu erstellen.

Ein häufiger Anwendungsfall ist die Zugriffskontrolle.

Hinweise:

- Wenn Sie einen Kommentar in einem Kommentar-Thread hinterlassen oder die API aufgerufen haben, um einen `Comment` zu erstellen, haben Sie bereits ein `Page`-Objekt erstellt! Sie können versuchen, es über
  die `/by-url-id` `Page`-Route abzurufen, indem Sie dieselbe `urlId` übergeben, die an das Kommentar-Widget übergeben wurde.
- Die `Page`-Struktur enthält einige **berechnete** Werte.
  Derzeit sind dies `commentCount` und `rootCommentCount`.
  Sie werden automatisch befüllt und können nicht über die API gesetzt werden. Der Versuch, dies zu tun, führt dazu, dass die API einen Fehler zurückgibt.

[inline-code-attrs-start title = 'Seite POST cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Seite POST Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Seite POST Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
