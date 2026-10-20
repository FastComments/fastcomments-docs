[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Denne rute flytter kommentarer og sider fra én `urlId` eller domæne til en anden, på samme måde som [Domain Migration tool](/guide-migrations.html#migrating-domains) i kontrolpanelet. Op til 100 migrationer kan sendes pr. anmodning. De udføres i den angivne rækkefølge, så `a` til `b` efterfulgt af `b` til `c` fungerer.

Hver migration har en `type`:

- `urlId` flytter siden hvis `urlId` er præcis `from`. `post-1` matcher ikke `post-10`.
- `domain` finder/erstatter `from` i hver `urlId` og `url`, og opdaterer `domain` for hver kommentar. Begge værdier skal være et domæne, f.eks. `old.example.com`.

Kommentarer, siden, side-reaktioner, stemmer, notifikationer og abonnementer flyttes alle. Hvis en side allerede findes på `to`, bliver de to sider flettet.

Migrationen kører som et batch-job. Svaret indeholder en `batchJobId`. Poll [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) for at følge dens fremdrift. Når jobbet er færdigt, har det et resultat pr. migration, og et `undoId`, som du kan sende til [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) inden for 90 dage.

Anmodningen valideres, før noget køres. Hvis en migration er ugyldig, fejler hele anmodningen med en `422`, og intet udføres.

Køning af en migration koster intet. Når jobbet er færdigt, koster det `1` kredit pr. `1000` kommentarer migreret, med et minimum på `1`.

[inline-code-attrs-start title = 'Page Migrations cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Page Migrations anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Migrations svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Jobs for én lejer kører én ad gangen, ældste først. En anden migration venter på at den første er færdig.
- `PATCH /api/v1/pages/:id` med en ny `urlId` ændrer kun siden, ikke dens kommentarer. Brug denne rute til at flytte en sides kommentarer.