[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Ova ruta poništava svaku migraciju iz jednog [`POST /api/v1/pages/migrations`](#pages-migrations-post) posla. Preuzmite `undoId` iz završenog posla migracije pomoću [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` se može koristiti 90 dana i samo jednom.

Poništavanje se izvršava kao batch posao, isto kao i migracija, a odgovor sadrži njegov `batchJobId`. Kada je posao završen, njegov `pageMigrationUndoResult` izveštava:

- `restored`: zapisi vraćeni na njihove vrednosti pre migracije.
- `skipped`: zapisi ostavljeni nepromenjeni jer su se promenili posle migracije, ili više ne postoje. Kasnija izmena se nikada ne prepisuje.
- `notRestorable`: zapisi koje je migracija izbrisala prilikom spajanja dve stranice ili uklanjanja duplih reakcija na stranicu. Ovi se ne mogu vratiti.

Migracije se poništavaju najnovije prvo, tako da se lanac poput `a` do `b` praćen `b` do `c` odmotava nazad do `a`. Posao migracije koji je neuspeo usred procesa i dalje se može poništiti, što vraća promene koje je napravio pre neuspeha.

Stavljanje poništavanja u red ne košta ništa. Kada posao završi, košta `1` kredit po `1000` vraćenih zapisa, sa minimalno `1`.

[inline-code-attrs-start title = 'Primer cURL zahteva za poništavanje migracija stranica'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za poništavanje migracija stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za poništavanje migracija stranica'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Uključeno pri grešci.
     * undo-not-found (404): nijedna migracija nema ovaj undoId, ili je stariji od 90 dana.
     * already-undone (409) i undo-in-progress (409): migracija je već poništena, ili je poništavanje u redu ili se izvršava.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Uključeno pri grešci. **/
    reason?: string
    /** Prosledite GET /api/v1/batch-jobs/:id za napredak i rezultat. **/
    batchJobId?: string
}
[inline-code-end]