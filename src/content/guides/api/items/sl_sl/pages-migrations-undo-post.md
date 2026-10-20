[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Ta pot obrne vsako migracijo iz enega [`POST /api/v1/pages/migrations`](#pages-migrations-post) posla. Pridobite `undoId` iz končanega migracijskega posla z [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` je mogoče uporabiti 90 dni in le enkrat.

Razveljavitev se izvede kot paketni posel, enako kot migracija, in odgovor vsebuje njegov `batchJobId`. Ko je posel končan, njegov `pageMigrationUndoResult` poroča:

- `restored`: zapisi, ki so vrnjeni na svoje vrednosti pred migracijo.
- `skipped`: zapisi, ki so ostali nespremenjeni, ker so se po migraciji spremenili ali več ne obstajajo. Kasnejša urejanja se nikoli ne prepišejo.
- `notRestorable`: zapisi, ki jih je migracija izbrisala med združevanjem dveh strani ali odstranjevanjem podvojenih odzivov strani. Ti se ne morejo povrniti.

Migracije se razveljavljajo najprej najnovejše, tako da se veriga, kot je `a` do `b`, sledena z `b` do `c`, razvozi nazaj do `a`. Migracijski posel, ki je delno spodletel, je še vedno mogoče razveljaviti, kar obrne spremembe, ki jih je naredil pred napako.

Postavitev razveljavitve v čakalno vrsto ne stane nič. Ko se posel konča, stane `1` kredit na `1000` obnovljenih zapisov, z minimalno ceno `1`.

[inline-code-attrs-start title = 'Razveljavi migracije strani cURL primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za razveljavitev migracij strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za razveljavitev migracij strani'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Vključeno ob napaki.
     * undo-not-found (404): nobena migracija nima tega undoId, ali je starejša od 90 dni.
     * already-undone (409) in undo-in-progress (409): migracija je bila razveljavljena, ali je razveljavitev v čakanju ali v teku.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Posredujte v GET /api/v1/batch-jobs/:id za napredek in rezultat. **/
    batchJobId?: string
}
[inline-code-end]