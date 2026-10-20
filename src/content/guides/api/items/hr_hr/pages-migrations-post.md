[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Ova ruta premješta komentare i stranice s jednog `urlId` ili domene na drugi, isto kao i [alat za migraciju domena](/guide-migrations.html#migrating-domains) u nadzornoj ploči. Može se poslati najviše 100 migracija po zahtjevu. Izvršavaju se redoslijedom navedenim, pa `a` na `b` praćeno `b` na `c` funkcionira.

Svaka migracija ima `type`:

- `urlId` premješta stranicu čiji je `urlId` točno `from`. `post-1` ne podudara se s `post-10`.
- `domain` pronalazi/ zamjenjuje `from` unutar svakog `urlId` i `url`, i ažurira `domain` svakog komentara. Obje vrijednosti moraju biti domena, poput `old.example.com`.

Komentari, stranica, reakcije na stranicu, glasovi, obavijesti i pretplate također se premještaju. Ako stranica već postoji na `to`, dvije stranice se spajaju.

Migracija se izvršava kao batch posao. Odgovor sadrži `batchJobId`. Pitanjem [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) pratite njegov napredak. Kada posao završi, ima rezultat po migraciji i `undoId` koji možete proslijediti na [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) u roku od 90 dana.

Zahtjev se provjerava prije nego što se nešto stavi u red. Ako je neka migracija neispravna, cijeli zahtjev ne uspijeva s kodom `422` i ništa se ne izvršava.

Stavljanje migracije u red ne košta ništa. Kada posao završi, košta `1` kredit po `1000` premještenih komentara, s minimalno `1`.

[inline-code-attrs-start title = 'Primjer cURL za migracije stranica'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za migracije stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Izvršava se redoslijedom navedenim. Najviše 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** Najviše 1000 znakova. Vodeći i završni razmak se uklanjaju. **/
    from: string
    /** Mora biti različito od "from". Najviše 1000 znakova. **/
    to: string
    /** "urlId" podudara se samo s točnim urlId. "domain" pronalazi/ zamjenjuje vrijednost unutar svakog urlId i url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za migracije stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Uključeno pri neuspjehu. Za invalid-migration, ovo navodi indeks prve neispravne migracije. **/
    reason?: string
    /** Proslijedite GET /api/v1/batch-jobs/:id za napredak i rezultat. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Poslovi za jednog najmodavca (tenant) izvršavaju se jedan po jedan, najstariji prvi. Druga migracija čeka da prva završi.
- `PATCH /api/v1/pages/:id` s novim `urlId` mijenja samo stranicu, a ne njene komentare. Koristite ovu rutu za premještanje komentara stranice.