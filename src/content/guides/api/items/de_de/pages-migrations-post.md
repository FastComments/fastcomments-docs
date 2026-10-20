[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Diese Route verschiebt Kommentare und Seiten von einer `urlId` oder Domain zu einer anderen, analog zum [Domain Migration tool](/guide-migrations.html#migrating-domains) im Dashboard. Pro Anfrage können bis zu 100 Migrationen gesendet werden. Sie werden in der angegebenen Reihenfolge ausgeführt, sodass `a` nach `b` gefolgt von `b` nach `c` funktioniert.

Jede Migration hat einen `type`:

- `urlId` verschiebt die Seite, deren `urlId` exakt `from` ist. `post-1` stimmt nicht mit `post-10` überein.
- `domain` sucht/ersetzt `from` in jedem `urlId` und `url` und aktualisiert die `domain` jedes Kommentars. Beide Werte müssen eine Domain sein, z. B. `old.example.com`.

Kommentare, die Seite, Seitenreaktionen, Stimmen, Benachrichtigungen und Abonnements werden alle verschoben. Wenn bereits eine Seite unter `to` existiert, werden die beiden Seiten zusammengeführt.

Die Migration wird als Batch-Job ausgeführt. Die Antwort enthält eine `batchJobId`. Abfragen von [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) ermöglichen das Verfolgen des Fortschritts. Sobald der Job abgeschlossen ist, gibt es ein Ergebnis pro Migration sowie eine `undoId`, die Sie innerhalb von 90 Tagen an [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) übergeben können.

Die Anfrage wird validiert, bevor etwas in die Warteschlange gestellt wird. Wenn eine Migration ungültig ist, schlägt die gesamte Anfrage mit einem `422` fehl und es wird nichts ausgeführt.

Das Einreihen einer Migration kostet nichts. Wenn der Job abgeschlossen ist, kostet es `1` Credit pro `1000` migrierte Kommentare, mindestens jedoch `1`.

[inline-code-attrs-start title = 'Beispiel für Page Migrations cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Migrations-Anfrage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** In der angegebenen Reihenfolge ausführen. Maximal 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** Höchstens 1000 Zeichen. Führende und nachfolgende Leerzeichen werden entfernt. **/
    from: string
    /** Muss sich von "from" unterscheiden. Höchstens 1000 Zeichen. **/
    to: string
    /** "urlId" stimmt nur mit der genauen urlId überein. "domain" sucht/ersetzt den Wert in jedem urlId und url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Page Migrations-Antwort'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Bei einem Fehler enthalten. Für invalid-migration gibt dies den Index der ersten ungültigen Migration an. **/
    reason?: string
    /** An GET /api/v1/batch-jobs/:id übergeben, um den Fortschritt und das Ergebnis zu erhalten. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Jobs für einen Mandanten werden nacheinander ausgeführt, zuerst die ältesten. Eine zweite Migration wartet, bis die erste abgeschlossen ist.
- `PATCH /api/v1/pages/:id` mit einer neuen `urlId` ändert nur die Seite, nicht deren Kommentare. Verwenden Sie diese Route, um die Kommentare einer Seite zu verschieben.

---