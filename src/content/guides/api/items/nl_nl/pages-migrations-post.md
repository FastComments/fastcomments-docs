[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Deze route verplaatst opmerkingen en pagina's van één `urlId` of domein naar een ander, hetzelfde als de [Domain Migration tool](/guide-migrations.html#migrating-domains) in het dashboard. Tot 100 migraties kunnen per verzoek worden verzonden. Ze worden uitgevoerd in de opgegeven volgorde, dus `a` naar `b` gevolgd door `b` naar `c` werkt.

Elke migratie heeft een `type`:

- `urlId` verplaatst de pagina waarvan de `urlId` exact `from` is. `post-1` komt niet overeen met `post-10`.
- `domain` zoekt/vervangt `from` binnen elke `urlId` en `url`, en werkt het `domain` van elk commentaar bij. Beide waarden moeten een domein zijn, zoals `old.example.com`.

Opmerkingen, de pagina, paginareacties, stemmen, meldingen en abonnementen worden allemaal verplaatst. Als er al een pagina bestaat op `to`, worden de twee pagina's samengevoegd.

De migratie wordt uitgevoerd als een batchtaak. Het antwoord bevat een `batchJobId`. Poll [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) om de voortgang te volgen. Zodra de taak is voltooid, heeft deze een resultaat per migratie, en een `undoId` die je binnen 90 dagen kunt doorgeven aan [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post).

Het verzoek wordt gevalideerd voordat iets in de wachtrij wordt geplaatst. Als een migratie ongeldig is, faalt het hele verzoek met een `422` en wordt er niets uitgevoerd.

Het in de wachtrij plaatsen van een migratie kost niets. Wanneer de taak voltooid is, kost het `1` credit per `1000` verplaatste opmerkingen, met een minimum van `1`.

[inline-code-attrs-start title = 'Voorbeeld cURL voor Pagina Migraties'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van Pagina Migraties Verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Voer uit in de opgegeven volgorde. Maximaal 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** Maximaal 1000 tekens. Voorloop- en naloopspaties worden verwijderd. **/
    from: string
    /** Moet verschillen van "from". Maximaal 1000 tekens. **/
    to: string
    /** "urlId" komt alleen overeen met de exacte urlId. "domain" zoekt/vervangt de waarde binnen elke urlId en url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Pagina Migraties Antwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Wordt opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Wordt opgenomen bij een fout. Voor invalid-migration wordt hiermee de index van de eerste ongeldige migratie weergegeven. **/
    reason?: string
    /** Geef door aan GET /api/v1/batch-jobs/:id voor voortgang en het resultaat. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Taken voor één tenant worden één voor één uitgevoerd, oudste eerst. Een tweede migratie wacht tot de eerste is voltooid.
- `PATCH /api/v1/pages/:id` met een nieuwe `urlId` wijzigt alleen de pagina, niet de opmerkingen. Gebruik deze route om de opmerkingen van een pagina te verplaatsen.