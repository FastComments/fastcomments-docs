[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Diese API wird verwendet, um Kommentare zur Anzeige für einen Benutzer abzurufen. Sie filtert beispielsweise automatisch
nicht genehmigte oder Spam-Kommentare heraus.

### Paginierung

Die Paginierung kann auf zwei Arten erfolgen, abhängig von den Leistungsanforderungen und dem Anwendungsfall:

1. Am schnellsten: **Vorberechnete Paginierung**:
   1. So funktioniert FastComments, wenn Sie unsere vorgefertigten Widgets und Clients verwenden.
   2. Ein Klick auf "weiter" erhöht einfach die Seitenzahl.
   3. Sie können sich das so vorstellen, als würde es durch einen Key-Value-Speicher abgerufen.
   4. Definieren Sie auf diese Weise einfach einen `page`-Parameter beginnend bei `0` und eine Sortierrichtung als `direction`.
   5. Seitengrößen können über Anpassungsregeln konfiguriert werden.
2. Am flexibelsten: **Flexible Paginierung**:
   1. Auf diese Weise können Sie benutzerdefinierte `limit`- und `skip`-Parameter definieren. Übergeben Sie nicht `page`.
   2. Sortierrichtung `direction` wird ebenfalls unterstützt.
   3. `limit` ist die Gesamtzahl, die nach Anwendung von `skip` zurückgegeben wird.
      - Beispiel: Setzen Sie `skip = 200, limit = 100` bei `Seitengröße = 100` und `page = 2`.
   4. Untergeordnete Kommentare zählen weiterhin bei der Paginierung. Sie können dies mit der Option `asTree` umgehen.
      - Sie können untergeordnete Kommentare über `limitChildren` und `skipChildren` paginieren.
      - Sie können die Tiefe der zurückgegebenen Threads über `maxTreeDepth` begrenzen.

### Threads

1. Bei Verwendung der `Vorberechneten Paginierung` werden Kommentare nach *Seite* gruppiert und Kommentare in Threads beeinflussen die Gesamtseite.
   1. Auf diese Weise können Threads auf dem Client basierend auf `parentId` bestimmt werden.
   2. Beispiel: Bei einer Seite mit einem Kommentar der obersten Ebene und 29 Antworten und der Einstellung `page=0` in der API - erhalten Sie nur den Kommentar der obersten Ebene und die 29 untergeordneten.
   3. [Beispielbild hier, das mehrere Seiten illustriert.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. Bei Verwendung der `Flexiblen Paginierung` können Sie einen `parentId`-Parameter definieren.
   1. Setzen Sie diesen auf null, um nur Kommentare der obersten Ebene zu erhalten.
   2. Um dann Threads anzuzeigen, rufen Sie die API erneut auf und übergeben `parentId`.
   3. Eine gängige Lösung ist, einen API-Aufruf für die Kommentare der obersten Ebene zu machen und dann parallele API-Aufrufe für die untergeordneten Kommentare jedes Kommentars zu machen.
3. __NEU seit Feb 2023!__ Als Baum abrufen mit `&asTree=true`.
   1. Sie können sich das als `Flexible Paginierung als Baum` vorstellen.
   2. Nur die Kommentare der obersten Ebene zählen bei der Paginierung.
   3. Setzen Sie `parentId=null`, um den Baum am Wurzelknoten zu beginnen (Sie müssen `parentId` setzen).
   4. Setzen Sie `skip` und `limit` für die Paginierung.
   5. Setzen Sie `asTree` auf `true`.
   6. Die Credit-Kosten erhöhen sich um das `2-fache`, da unser Backend in diesem Szenario viel mehr Arbeit leisten muss.
   7. Setzen Sie `maxTreeDepth`, `limitChildren` und `skipChildren` nach Bedarf.

### Bäume erklärt

Bei Verwendung von `asTree` kann es schwierig sein, die Paginierung nachzuvollziehen. Hier ist eine hilfreiche Grafik:

<div class="screenshot white-bg">
    <div class="title">Baum-Paginierung Diagramm</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Baum-Paginierung Diagramm" />
</div>

### Kommentare im Kontext eines Benutzers abrufen

Die `/comments`-API kann in zwei Kontexten für verschiedene Anwendungsfälle verwendet werden:

- Für die Rückgabe von Kommentaren, sortiert und mit Informationen versehen, um Ihren eigenen Client zu erstellen.
  - Definieren Sie in diesem Fall einen `contextUserId`-Abfrageparameter.
- Zum Abrufen von Kommentaren von Ihrem Backend für benutzerdefinierte Integrationen.
  - Die Plattform verwendet standardmäßig dies ohne `contextUserId`.

[inline-code-attrs-start title = 'Kommentare Vorberechnete Paginierung'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentare Flexible Paginierung'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentare Flexible Paginierung im Benutzerkontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Kommentare Flexible Paginierung im Benutzerkontext nur für Kommentare der obersten Ebene'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Kommentare als Baum abrufen

Es ist möglich, die Kommentare als Baum zurückzugeben, wobei die Paginierung nur die Kommentare der obersten Ebene zählt.

[inline-code-attrs-start title = 'Kommentare als Baum im Benutzerkontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Möchten Sie nur die Kommentare der obersten Ebene und die unmittelbaren Kinder erhalten? Hier ist eine Möglichkeit:

[inline-code-attrs-start title = 'Kommentare als Baum mit maximaler Tiefe'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

In Ihrer Benutzeroberfläche müssen Sie jedoch möglicherweise wissen, ob bei
jedem Kommentar eine Schaltfläche "Antworten anzeigen" angezeigt werden soll. Beim Abrufen von Kommentaren über einen Baum gibt es eine `hasChildren`-Eigenschaft, die
bei Bedarf an Kommentare angehängt wird.

### Kommentare als Baum abrufen, Suche nach Hash-Tag

Es ist möglich, mit der API nach Hashtag zu suchen, über Ihren gesamten Tenant (nicht auf eine Seite oder `urlId` beschränkt).

In diesem Beispiel lassen wir `urlId` weg und suchen nach mehreren Hashtags. Die API gibt nur Kommentare zurück, die alle angeforderten Hashtags haben.

[inline-code-attrs-start title = 'Kommentare als Baum im Benutzerkontext, nach Hash-Tag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle Anfrageparameter

[inline-code-attrs-start title = 'Kommentare Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The urlId (page url, or article id) the comments are associated with. **/
    urlId?: string
    /** Limit the comments returned by this user. **/
    userId?: string
    /** Use this to search by hashtag. To drill down to the intersection of multiple hashtags, do &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** The sort direction. Default is MR (Most Relevant). Other options are OF (Oldest First) and NF (Newest First). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Precalculated Pagination: The page to fetch, starting with 0. Pass -1 for all comments (up to 250). **/
    page?: number
    /** Flexible Pagination: How many comments should we return? **/
    limit?: number
    /** Flexible Pagination: How many child comments should we return for each parent? **/
    limitChildren?: number
    /** Flexible Pagination: How many comments should we skip? **/
    skip?: number
    /** Flexible Pagination: How many child comments should we skip for each parent? **/
    skipChildren?: number
    /** For determining blocked and flagged comments. **/
    contextUserId?: string
    /** For determining blocked and flagged comments. **/
    anonUserId?: string
    /** For fetching child comments. **/
    parentId?: string
    /** For fetching as a tree. **/
    asTree?: boolean
    /** How far into the tree should we return data? 0 returns no children. 1 returns immediate children, etc. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Die Antwort

[inline-code-attrs-start title = 'Kommentare Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Included on failure. **/
    reason?: string
    /** The comments! **/
    comments: Comment[]
}
[inline-code-end]

### Hilfreiche Tipps

#### URL-ID

Sie möchten wahrscheinlich die `Comment`-API mit dem `urlId`-Parameter verwenden. Sie können zuerst die `Pages`-API aufrufen, um zu sehen, wie die verfügbaren `urlId`-Werte aussehen.

#### Anonyme Aktionen

Für anonymes Kommentieren möchten Sie wahrscheinlich `anonUserId` beim Abrufen von Kommentaren und beim Durchführen von Markierungen und Blockierungen übergeben.

(!) Dies ist für viele App-Stores erforderlich, da Benutzer in der Lage sein müssen, benutzergenerierten Inhalt, den sie sehen können, zu markieren, auch wenn sie nicht angemeldet sind. Wenn Sie dies nicht tun, kann Ihre App aus dem entsprechenden Store entfernt werden.

#### Kommentare werden nicht zurückgegeben

Überprüfen Sie, ob Ihre Kommentare genehmigt sind und kein Spam sind.
