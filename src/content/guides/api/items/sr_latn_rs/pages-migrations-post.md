[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Ova ruta premešta komentare i stranice sa jednog `urlId`‑a ili domena na drugi, isto kao i [Alat za migraciju domena](/guide-migrations.html#migrating-domains) u kontrolnoj tabli. Do 100 migracija može se poslati po zahtevu. One se izvršavaju redom kojim su navedene, pa `a` na `b` praćeno `b` na `c` funkcioniše.

Svaka migracija ima `type`:

- `urlId` premešta stranicu čiji je `urlId` tačno `from`. `post-1` ne odgovara `post-10`.
- `domain` pronalazi/zamenjuje `from` unutar svakog `urlId` i `url`, i ažurira `domain` svakog komentara. Obe vrednosti moraju biti domen, npr. `old.example.com`.

Komentari, stranica, reakcije na stranicu, glasovi, obaveštenja i pretplate se sve premeštaju. Ako stranica već postoji na `to`, dve stranice se spajaju.

Migracija se izvršava kao batch posao. Odgovor sadrži `batchJobId`. Pitanjem [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) možete pratiti njegov napredak. Kada posao završi, ima rezultat po migraciji, i `undoId` koji možete proslediti na [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) u roku od 90 dana.

Zahtev se validira pre nego što se nešto stavi u red. Ako je neka migracija neispravna, ceo zahtev ne uspeva sa `422` i ništa se ne izvršava.

Stavljanje migracije u red ne košta ništa. Kada posao završi, košta `1` kredit po `1000` premeštenih komentara, uz minimum od `1`.

[inline-code-attrs-start title = 'Primer cURL zahteva za migracije stranica'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahteva za migracije stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za migracije stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Napomene

- Poslovi za jednog zakupca se izvršavaju jedan po jedan, najstariji prvi. Druga migracija čeka da prva završi.
- `PATCH /api/v1/pages/:id` sa novim `urlId` menja samo stranicu, a ne njene komentare. Koristite ovu rutu da premestite komentare stranice.