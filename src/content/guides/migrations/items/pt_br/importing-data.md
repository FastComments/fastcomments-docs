---
Embora o suporte do FastComments possa ajudar nas migrações, a maioria pode ser realizada e monitorada facilmente sem qualquer intervenção da equipe de suporte.

Nós suportamos nativamente a importação de exportações dos seguintes provedores:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via o plugin, ou um export XML ou CSV)
- AnyComment (Via WordPress Import/Export)

Ao navegar [aqui](https://fastcomments.com/auth/my-account/manage-data/import) podemos fazer upload do arquivo contendo os dados a serem migrados.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='Página de importação do FastComments com a seleção de provedor e campos de upload de arquivo para um arquivo de exportação'; title='Formulário da página de importação' app-screenshot-end]

### Monitoramento de Importações

O FastComments usa um sistema de processamento de tarefas para processar importações e exportações. Uma vez que o sistema tenha capturado sua tarefa, ele relatará periodicamente o status da tarefa na interface de importação ou exportação.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Página de importação mostrando um trabalho de importação em execução e o status relatado pelo sistema de processamento de trabalhos'; title='Status do trabalho de importação' app-screenshot-end]

Observe que o status das importações e exportações pode ser visualizado por todos os administradores da conta.

Se sua tarefa falhar, ela não será reiniciada automaticamente. A importação terá que ser tentada novamente. Se alguma importação ou exportação falhar, nossos administradores de sistema são notificados automaticamente. Se identificarmos um problema, entraremos em contato para ver se podemos ajudar.

### Reexecutando a Importação

Durante algumas migrações, é necessário executar a importação várias vezes. Por exemplo, é comum fazer uma primeira passagem de migração para testes e, em seguida, executar a importação novamente com os dados mais recentes antes de mudar o sistema.

Reimportar o mesmo conteúdo **não criará duplicatas**.

### Segurança de Dados e Expiração

Os arquivos de importação não são acessíveis por solicitações externas de nenhuma forma, e os arquivos de importação são excluídos do nosso sistema assim que a importação é concluída.

---