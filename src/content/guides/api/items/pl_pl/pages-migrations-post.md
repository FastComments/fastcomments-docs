[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Ta trasa przenosi komentarze i strony z jednego `urlId` lub domeny do innej, tak jak narzędzie [Domain Migration tool](/guide-migrations.html#migrating-domains) w panelu. Można wysłać do 100 migracji na żądanie. Są wykonywane w podanej kolejności, więc migracja z `a` do `b` a następnie z `b` do `c` działa.

Każda migracja ma `type`:

- `urlId` przenosi stronę, której `urlId` jest dokładnie `from`. `post-1` nie pasuje do `post-10`.
- `domain` znajduje i zamienia `from` we wszystkich `urlId` i `url`, oraz aktualizuje `domain` każdego komentarza. Obie wartości muszą być domeną, np. `old.example.com`.

Komentarze, strona, reakcje na stronie, głosy, powiadomienia i subskrypcje są wszystkie przenoszone. Jeśli strona już istnieje pod `to`, dwie strony są łączone.

Migracja jest uruchamiana jako zadanie wsadowe. Odpowiedź zawiera `batchJobId`. Odpytywać [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id), aby śledzić postęp. Po zakończeniu zadania ma wynik dla każdej migracji oraz `undoId`, które można przekazać do [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) w ciągu 90 dni.

Żądanie jest walidowane przed umieszczeniem w kolejce. Jeśli którakolwiek migracja jest nieprawidłowa, całe żądanie kończy się niepowodzeniem z kodem `422` i nic nie jest wykonywane.

Umieszczenie migracji w kolejce nie kosztuje nic. Po zakończeniu zadania kosztuje `1` kredyt za każde `1000` przeniesionych komentarzy, z minimum `1`.

[inline-code-attrs-start title = 'Przykład cURL migracji stron'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania migracji stron'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odpowiedzi migracji stron'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Notatki

- Zadania dla jednego najemcy są wykonywane jedno po drugim, najstarsze najpierw. Druga migracja czeka, aż pierwsza się zakończy.
- `PATCH /api/v1/pages/:id` z nowym `urlId` zmienia tylko stronę, a nie jej komentarze. Użyj tej trasy, aby przenieść komentarze strony.