Caso seja necessário mover dados, o FastComments oferece uma ferramenta de autoatendimento para mover comentários entre páginas e artigos.

Veja como é o formulário de cópia de comentários da página:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Formulário de cópia de comentários com o campo From URL ID e os campos To URL ID e URL'; title='O Formulário de Cópia de Comentário' app-screenshot-end]

### Preenchendo os Campos "From"

Para decidir de onde mover os comentários, precisamos apenas saber o `URL ID` de origem.

Se você não estiver passando um valor para `urlId` na configuração do widget de comentários, então isso será uma versão "limpa" da URL da página.

Você pode ver quais valores seus comentários têm para `URL ID` exportando‑os.

### Preenchendo os Campos "To"

Para decidir para onde mover os comentários, precisamos conhecer o `URL ID` e a `URL` de destino.

O `URL ID` será o contêiner onde o comentário será colocado. O campo `URL` é usado para que você possa navegar diretamente ao comentário a partir de e‑mails e ferramentas de moderação.

#### WordPress

Se você estiver usando WordPress, por exemplo, inseriria os IDs dos artigos nos campos `URL ID` To/From na ferramenta de migração, em vez de uma URL.