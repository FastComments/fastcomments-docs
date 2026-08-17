[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Diese API wird verwendet, um Kommentare zum Anzeigen für einen Benutzer abzurufen. Beispielsweise filtert sie automatisch nicht genehmigte oder Spam‑Kommentare heraus.

### Paginierung

Die Paginierung kann auf eine von zwei Arten erfolgen, abhängig von Leistungsanforderungen und Anwendungsfall:

1. Schnellste: **Vorkalkulierte Paginierung**:
   1. So funktioniert FastComments, wenn Sie unsere vorgefertigten Widgets und Clients verwenden.
   2. Durch Klicken auf „Weiter“ wird einfach die Seitenzahl erhöht.
   3. Man kann sich das vorstellen, als würde es aus einem Schlüssel‑Wert‑Speicher abgerufen.
   4. Auf diese Weise definieren Sie einfach einen `page`‑Parameter, beginnend bei `0`, und eine Sortierrichtung als `direction`.
   5. Seitengrößen können über Anpassungsregeln angepasst werden.
2. Flexibelste: **Flexible Paginierung**:
   1. Auf diese Weise können Sie benutzerdefinierte `limit`‑ und `skip`‑Parameter festlegen. Übergeben Sie nicht `page`.
   2. Die Sortierrichtung `direction` wird ebenfalls unterstützt.
   3. `limit` ist die Gesamtzahl, die nach Anwendung von `skip` zurückgegeben wird.
      - Beispiel: Setzen Sie `skip = 200, limit = 100`, wenn `page size = 100` und `page = 2`.
   4. Unterkommentare zählen weiterhin in die Paginierung. Sie können dies umgehen, indem Sie die Option `asTree` verwenden.
      - Sie können Unterkommentare über `limitChildren` und `skipChildren` paginieren.
      - Sie können die Tiefe der zurückgegebenen Threads über `maxTreeDepth` begrenzen.

### Threads

1. Bei Verwendung von `Precalculated Pagination` werden Kommentare nach *Seite* gruppiert und Kommentare in Threads beeinflussen die gesamte Seite.
   1. Auf diese Weise können Threads auf dem Client anhand von `parentId` bestimmt werden.
   2. Zum Beispiel, bei einer Seite mit einem Top‑Level‑Kommentar und 29 Antworten und `page=0` in der API – Sie erhalten nur den Top‑Level‑Kommentar und die 29 Unterkommentare.
2. Bei Verwendung von `Flexible Pagination` können Sie einen `parentId`‑Parameter definieren.
   1. Setzen Sie diesen auf null, um nur Top‑Level‑Kommentare zu erhalten.
   2. Rufen Sie dann die API erneut auf und übergeben Sie `parentId`, um Threads anzuzeigen.
   3. Eine gängige Lösung besteht darin, einen API‑Aufruf für die Top‑Level‑Kommentare zu machen und dann parallele API‑Aufrufe, um Kommentare für die Unterkommentare jedes Kommentars zu erhalten.
3. __NEU seit Feb 2023!__ Abrufen als Baum mit `&asTree=true`.
   1. Man kann sich das vorstellen als `Flexible Pagination als Baum`.
   2. Nur die Top‑Level‑Kommentare zählen in der Paginierung.
   3. Setzen Sie `parentId=null`, um den Baum an der Wurzel zu starten (Sie müssen `parentId` setzen).
   4. Setzen Sie `skip` und `limit` für die Paginierung.
   5. Setzen Sie `asTree` auf `true`.
   6. Die Kosten in Credits erhöhen sich um `2x`, da unser Backend in diesem Szenario viel mehr Arbeit leisten muss.
   7. Setzen Sie `maxTreeDepth`, `limitChildren` und `skipChildren` nach Wunsch.

### Bäume erklärt

Bei Verwendung von `asTree` kann es schwierig sein, die Paginierung zu verstehen. Hier ist eine praktische Grafik:

<div class="screenshot white-bg">
    <div class="title">Baum‑Paginierungsdiagramm</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Baum‑Paginierungsdiagramm" />
</div>

### Abrufen von Kommentaren im Kontext eines Benutzers

Die `/comments`‑API kann in zwei Kontexten für unterschiedliche Anwendungsfälle verwendet werden:

- Zum Zurückgeben von Kommentaren, sortiert und mit Informationen versehen, um Ihren eigenen Client zu bauen.
  - In diesem Fall definieren Sie einen `contextUserId`‑Abfrageparameter.
- Zum Abrufen von Kommentaren von Ihrem Backend für benutzerdefinierte Integrationen.
  - Die Plattform verwendet standardmäßig dies ohne `contextUserId`.

[inline-code-attrs-start title = 'Kommentare Vorkalkulierte Paginierung'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Kommentare Flexible Paginierung im Benutzerkontext nur für Top-Level-Kommentare'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Kommentare als Baum abrufen

Es ist möglich, die zurückgegebenen Kommentare als Baum zu erhalten, wobei die Paginierung nur die Top‑Level‑Kommentare zählt.

[inline-code-attrs-start title = 'Kommentare Als‑Baum im Benutzerkontext'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Möchten Sie nur die Top‑Level‑Kommentare und die unmittelbaren Unterkommentare erhalten? Hier ist ein Weg:

[inline-code-attrs-start title = 'Kommentare Als‑Baum mit maximaler Tiefe'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Allerdings müssen Sie in Ihrer UI möglicherweise wissen, ob für jeden Kommentar ein „Antworten anzeigen“-Button angezeigt werden soll. Beim Abrufen von Kommentaren über einen Baum wird, falls zutreffend, eine `hasChildren`‑Eigenschaft an die Kommentare angehängt.

### Kommentare als Baum abrufen, Suche nach Hashtag

Es ist möglich, über die API nach Hashtag zu suchen, über Ihren gesamten Mandanten hinweg (nicht auf eine Seite oder `urlId` beschränkt).

In diesem Beispiel lassen wir `urlId` weg und suchen nach mehreren Hashtags. Die API gibt nur Kommentare zurück, die alle angeforderten Hashtags enthalten.

[inline-code-attrs-start title = 'Kommentare Als‑Baum im Benutzerkontext, nach Hashtag'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Alle Anforderungsparameter

[inline-code-attrs-start title = 'Struktur der Kommentar-Anfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Die urlId (Seiten-URL oder Artikel-ID), mit der die Kommentare verknüpft sind. **/
    urlId?: string
    /** Begrenzen Sie die von diesem Benutzer zurückgegebenen Kommentare. **/
    userId?: string
    /** Verwenden Sie dies, um nach Hashtag zu suchen. Um die Schnittmenge mehrerer Hashtags zu erhalten, verwenden Sie &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Die Sortierrichtung. Standard ist MR (Meistrelevant). Weitere Optionen sind OF (Älteste zuerst) und NF (Neueste zuerst). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Vorkalkulierte Paginierung: Die abzurufende Seite, beginnend bei 0. Verwenden Sie -1 für alle Kommentare (bis zu 250). **/
    page?: number
    /** Flexible Paginierung: Wie viele Kommentare sollen zurückgegeben werden? **/
    limit?: number
    /** Flexible Paginierung: Wie viele Unterkommentare sollen pro übergeordnetem Kommentar zurückgegeben werden? **/
    limitChildren?: number
    /** Flexible Paginierung: Wie viele Kommentare sollen übersprungen werden? **/
    skip?: number
    /** Flexible Paginierung: Wie viele Unterkommentare sollen pro übergeordnetem Kommentar übersprungen werden? **/
    skipChildren?: number
    /** Zur Bestimmung blockierter und gemeldeter Kommentare. **/
    contextUserId?: string
    /** Zur Bestimmung blockierter und gemeldeter Kommentare. **/
    anonUserId?: string
    /** Zum Abrufen von Unterkommentaren. **/
    parentId?: string
    /** Zum Abrufen als Baum. **/
    asTree?: boolean
    /** Wie tief in den Baum sollen Daten zurückgegeben werden? 0 gibt keine Unterkommentare zurück. 1 gibt unmittelbare Unterkommentare zurück usw. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Die Antwort

[inline-code-attrs-start title = 'Struktur der Kommentar-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Bei einem Fehler enthalten. **/
    reason?: string
    /** Die Kommentare! **/
    comments: Comment[]
}
[inline-code-end]

### Hilfreiche Tipps

#### URL-ID

Sie möchten wahrscheinlich die `Comment`‑API mit dem Parameter `urlId` verwenden. Sie können zuerst die `Pages`‑API aufrufen, um zu sehen, wie die für Sie verfügbaren `urlId`‑Werte aussehen.

#### Anonyme Aktionen

Für anonyme Kommentare sollten Sie wahrscheinlich `anonUserId` beim Abrufen von Kommentaren sowie beim Melden und Blockieren übergeben.

(!) Dies ist für viele App‑Stores erforderlich, da Benutzer in der Lage sein müssen, von ihnen sichtbare, von Benutzern erstellte Inhalte zu melden, selbst wenn sie nicht eingeloggt sind. Wenn Sie dies nicht tun, kann Ihre App aus dem jeweiligen Store entfernt werden.

#### Kommentare werden nicht zurückgegeben

Stellen Sie sicher, dass Ihre Kommentare genehmigt sind und nicht als Spam gelten.

---