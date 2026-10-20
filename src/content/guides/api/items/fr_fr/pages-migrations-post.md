[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Cette route déplace les commentaires et les pages d'un `urlId` ou d'un domaine vers un autre, de la même façon que l'[outil de migration de domaine](/guide-migrations.html#migrating-domains) dans le tableau de bord. Jusqu'à 100 migrations peuvent être envoyées par requête. Elles s'exécutent dans l'ordre indiqué, ainsi `a` vers `b` suivi de `b` vers `c` fonctionne.

Chaque migration possède un `type` :

- `urlId` déplace la page dont le `urlId` est exactement `from`. `post-1` ne correspond pas à `post-10`.
- `domain` recherche/remplace `from` dans chaque `urlId` et `url`, et met à jour le `domain` de chaque commentaire. Les deux valeurs doivent être un domaine, comme `old.example.com`.

Les commentaires, la page, les réactions de page, les votes, les notifications et les abonnements sont tous déplacés. Si une page existe déjà à `to`, les deux pages sont fusionnées.

La migration s'exécute en tant que tâche batch. La réponse contient un `batchJobId`. Interrogez [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) pour suivre sa progression. Une fois la tâche terminée, elle possède un résultat par migration, ainsi qu'un `undoId` que vous pouvez transmettre à [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) dans les 90 jours.

La requête est validée avant d'être mise en file d'attente. Si une migration est invalide, toute la requête échoue avec un `422` et rien ne s'exécute.

Mettre en file d'attente une migration ne coûte rien. Lorsque la tâche se termine, elle coûte `1` crédit par `1000` commentaires migrés, avec un minimum de `1`.

[inline-code-attrs-start title = 'Exemple cURL des migrations de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structure de la requête des migrations de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structure de la réponse des migrations de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

#### Remarques

- Les tâches pour un même locataire s'exécutent une à la fois, les plus anciennes d'abord. Une deuxième migration attend que la première se termine.
- `PATCH /api/v1/pages/:id` avec un nouveau `urlId` ne modifie que la page, pas ses commentaires. Utilisez cette route pour déplacer les commentaires d'une page.