FastComments fornece uma maneira automatizada para você migrar seus comentários entre domínios.

A migração de domínio simplesmente requer um domínio `from` e um domínio `to`.

Isso **move** comentários, não os copia. Se você deseja copiar comentários, entre em contato conosco.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Ferramenta de migração de domínio com os campos de domínio de origem e destino e a contagem de comentários migrados'; title='Migrando Domínios' app-screenshot-end]

Isso também é útil, por exemplo, se parte da sua migração para o FastComments envolve migrar de um provedor diferente, de modo que os dados de importação de comentários
possam conter informações que precisam ser migradas. Nesse caso, você pode executar a importação, e então a migração de domínio.

### Monitorando o Progresso

A ferramenta de Migração de Domínio usa o mesmo sistema de processamento de jobs do FastComments que as outras ferramentas de gerenciamento de dados.

Pode haver um atraso antes que sua migração comece. Isso é normal, pois o sistema verifica periodicamente novos jobs para processar.

À medida que o job é executado, ele exibirá o número de comentários encontrados para migração e o número de comentários migrados até o momento.

### Migrando Domínios via API

Domínios também podem ser migrados com [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post), usando `type` definido como `domain`. O progresso está disponível a partir de [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id), e a migração pode ser desfeita por 90 dias.

---