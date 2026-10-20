---
Embora seu nome seja a ferramenta de Migração de Domínio, ela também pode ser usada para migrar comentários entre páginas individuais.

Basta inserir os valores antigos e novos de [urlId](/guide-customizations-and-configuration.html#url-id) nos campos de entrada `from` e `to`.

### Migrando Páginas via API

Para migrar muitas páginas de uma vez, use [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). Ele aceita até 100 pares `from` e `to` por solicitação e os executa como um trabalho em lote.

Com `type` definido como `urlId`, somente a página cujo `urlId` é exatamente `from` é movida, portanto `post-1` não corresponde a `post-10`. A ferramenta do painel não possui essa opção.

Cada migração de API pode ser desfeita por 90 dias com [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---