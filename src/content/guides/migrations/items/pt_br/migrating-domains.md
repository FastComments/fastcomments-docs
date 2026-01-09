---
FastComments fornece uma forma automatizada para você migrar seus comentários entre domínios.

A migração de domínio simplesmente requer um `from` e um `to` domain.

Isso **move** comentários, não os copia. Se você desejar copiar comentários, entre em contato conosco.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; title='Migrating Domains' app-screenshot-end]

Isto também é útil, por exemplo, se parte da sua migração para o FastComments envolver migração a partir de um provedor diferente, então seus dados de importação
de comentários podem conter dados que precisam ser migrados. Neste caso, você pode executar a importação e, em seguida, a migração de domínios.

### Monitoramento do Progresso

A ferramenta de Migração de Domínios usa o mesmo sistema de processamento de jobs do FastComments que as outras ferramentas de gerenciamento de dados.

Pode haver um atraso antes de sua migração começar. Isso é normal, pois o sistema verifica periodicamente novos jobs para processar.

À medida que o job for executado, ele exibirá o número de comentários encontrados para migrar e o número migrado até o momento.

---