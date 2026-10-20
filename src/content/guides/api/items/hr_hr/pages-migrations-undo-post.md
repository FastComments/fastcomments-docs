[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Ova ruta poništava svaku migraciju iz jednog posla [`POST /api/v1/pages/migrations`](#pages-migrations-post). Preuzmite `undoId` iz dovršenog posla migracije pomoću [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` se može koristiti 90 dana i samo jednom.

Undo se izvršava kao batch posao, isto kao i migracija, a odgovor sadrži njegov `batchJobId`. Kada je posao dovršen, njegov `pageMigrationUndoResult` izvještava:

- `restored`: zapisi vraćeni na svoje vrijednosti prije migracije.
- `skipped`: zapisi ostavljeni nepromijenjeni jer su se promijenili nakon migracije ili više ne postoje. Kasnija izmjena se nikada ne prepisuje.
- `notRestorable`: zapisi koje je migracija izbrisala prilikom spajanja dviju stranica ili uklanjanja duplikata reakcija stranice. Ti se zapisi ne mogu vratiti.

Migracije se poništavaju najnovije prvo, tako da se lanac poput `a` do `b` praćen `b` do `c` odmotava natrag do `a`. Posao migracije koji je neuspješno prekinut može se i dalje poništiti, što vraća promjene koje je napravio prije neuspjeha.

Stavljanje poništavanja u red ne košta ništa. Kada posao završi, košta `1` kredit po `1000` vraćenih zapisa, s minimalno `1`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za poništavanje migracija stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za poništavanje migracija stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za poništavanje migracija stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

---