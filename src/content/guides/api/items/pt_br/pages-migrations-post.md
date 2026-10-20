[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Esta rota move comentários e páginas de um `urlId` ou domínio para outro, da mesma forma que a [Domain Migration tool](/guide-migrations.html#migrating-domains) no dashboard. Até 100 migrações podem ser enviadas por requisição. Elas são executadas na ordem fornecida, portanto `a` para `b` seguido de `b` para `c` funciona.

Cada migração tem um `type`:

- `urlId` move a página cujo `urlId` é exatamente `from`. `post-1` não corresponde a `post-10`.
- `domain` encontra/substitui `from` dentro de cada `urlId` e `url`, e atualiza o `domain` de cada comentário. Ambos os valores devem ser um domínio, como `old.example.com`.

Comentários, a página, reações da página, votos, notificações e assinaturas são todos movidos. Se uma página já existir em `to`, as duas páginas são mescladas.

A migração é executada como um job em lote. A resposta contém um `batchJobId`. Consulte [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) para acompanhar seu progresso. Quando o job terminar, ele terá um resultado por migração, e um `undoId` que pode ser passado para [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) dentro de 90 dias.

A requisição é validada antes de ser enfileirada. Se alguma migração for inválida, toda a requisição falha com um `422` e nada é executado.

Enfileirar uma migração não custa nada. Quando o job termina, custa `1` crédito por `1000` comentários migrados, com um mínimo de `1`.

[inline-code-attrs-start title = 'Exemplo cURL de Migrações de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estrutura de Requisição de Migrações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Execute na ordem fornecida. No máximo 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** No máximo 1000 caracteres. Espaços em branco iniciais e finais são removidos. **/
    from: string
    /** Deve ser diferente de "from". No máximo 1000 caracteres. **/
    to: string
    /** "urlId" só corresponde ao urlId exato. "domain" encontra/substitui o valor dentro de cada urlId e url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura de Resposta de Migrações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Incluído em caso de falha. Para invalid-migration, indica o índice da primeira migração inválida. **/
    reason?: string
    /** Passe para GET /api/v1/batch-jobs/:id para progresso e resultado. **/
    batchJobId?: string
}
[inline-code-end]

#### Notes

- Os trabalhos de um mesmo tenant são executados um de cada vez, do mais antigo para o mais recente. Uma segunda migração aguarda a conclusão da primeira.
- `PATCH /api/v1/pages/:id` com um novo `urlId` altera apenas a página, não seus comentários. Use esta rota para mover os comentários de uma página.