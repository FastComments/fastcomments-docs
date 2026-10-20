[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Ta trasa odwraca każdą migrację z jednego zadania [`POST /api/v1/pages/migrations`](#pages-migrations-post). Pobierz `undoId` z zakończonego zadania migracji za pomocą [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` może być użyte przez 90 dni i tylko raz.

Cofanie działa jako zadanie wsadowe, tak samo jak migracja, a odpowiedź zawiera jego `batchJobId`. Gdy zadanie zostanie zakończone, jego `pageMigrationUndoResult` raportuje:

- `restored`: rekordy przywrócone do wartości sprzed migracji.
- `skipped`: rekordy pozostawione bez zmian, ponieważ zmieniły się po migracji lub już nie istnieją. Późniejsza edycja nigdy nie zostanie nadpisana.
- `notRestorable`: rekordy, które migracja usunęła podczas scalania dwóch stron lub usuwania duplikatów reakcji strony. Nie można ich przywrócić.

Migracje są odwracane od najnowszych, więc łańcuch taki jak `a` do `b` a następnie `b` do `c` jest rozwijany z powrotem do `a`. Zadanie migracji, które nie powiodło się w połowie, może nadal zostać cofnięte, co odwraca zmiany wprowadzone przed awarią.

Umieszczenie cofania w kolejce nie kosztuje nic. Gdy zadanie się zakończy, kosztuje `1` kredyt za każde `1000` przywróconych rekordów, z minimum `1`.

[inline-code-attrs-start title = 'Przykład cURL Cofania Migracji Stron'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Cofania Migracji Stron'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Cofania Migracji Stron'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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