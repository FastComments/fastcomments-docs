Embora o Suporte do FastComments possa ajudar com migrações, a maioria pode ser executada e monitorada facilmente sem qualquer intervenção
da equipe de suporte.

Suportamos nativamente a importação de arquivos de exportação dos seguintes provedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via o plugin)

Ao navegar [here](https://fastcomments.com/auth/my-account/manage-data/import) podemos enviar o arquivo contendo os dados a serem migrados.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments uses a job processing system for processing imports and exports. Once the system has picked up your job, it will
periodically report the status of the job in the import or export UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Observe que o status de importações e exportações é visível para todos os administradores da conta.

Se o seu job falhar, ele não será reiniciado automaticamente. A importação terá que ser tentada novamente. Se alguma importação ou exportação falhar,
nossos administradores do sistema são notificados automaticamente. Se identificarmos um problema, entraremos em contato com você para ver se podemos ajudar.

### Re-Running The Import

Durante algumas migrações, é necessário executar a importação várias vezes. Por exemplo, é comum fazer uma primeira passagem
de migração para testes, e então executar a importação novamente com os dados mais recentes antes de ativar a mudança.

Reimportar o mesmo conteúdo **não criará duplicatas**.

### Data Security and Expiration

Os arquivos de importação não são acessíveis por requisições externas de forma alguma, e os arquivos de importação são excluídos do nosso sistema assim que
a importação for concluída.