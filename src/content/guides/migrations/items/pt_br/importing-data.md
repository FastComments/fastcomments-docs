---
Embora o Suporte do FastComments possa ajudar com migrações, a maioria pode ser realizada e monitorada facilmente sem qualquer intervenção da equipe de suporte.

Oferecemos suporte nativo para importar arquivos de exportação dos seguintes provedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via o plugin)
- AnyComment (Via WordPress Import/Export)

Ao navegar [aqui](https://fastcomments.com/auth/my-account/manage-data/import) podemos fazer o upload do arquivo contendo os dados para migrar.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoramento de Importações

O FastComments usa um sistema de processamento de jobs para processar importações e exportações. Uma vez que o sistema iniciar o processamento da sua tarefa, ele reportará periodicamente o status da tarefa na interface de importação ou exportação.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Observe que o status de Importações e Exportações pode ser visualizado por todos os administradores da conta.

Se sua tarefa falhar, ela não será reiniciada automaticamente. A importação terá que ser tentada novamente. Se qualquer importação ou exportação falhar, nossos administradores de sistema são notificados automaticamente. Se identificarmos um problema, entraremos em contato com você para ver se podemos ajudar.

### Re-executando a Importação

Durante algumas migrações, é necessário executar a importação várias vezes. Por exemplo, é comum fazer uma primeira migração para testes e, em seguida, executar a importação novamente com os dados mais recentes antes de colocar em produção.

Reimportar o mesmo conteúdo **não criará duplicatas**.

### Segurança dos Dados e Expiração

Os arquivos de importação não são acessíveis por requisições externas de nenhuma forma, e os arquivos de importação são excluídos do nosso sistema assim que a importação é concluída.

---