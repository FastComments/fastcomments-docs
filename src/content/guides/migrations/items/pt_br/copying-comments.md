Caso seja necessário mover dados, o FastComments oferece uma ferramenta de autoatendimento para mover comentários entre páginas e artigos.

Veja como é o formulário de cópia de comentários:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Preenchendo os campos "De"

Para decidir de onde mover os comentários, precisamos apenas saber o `URL ID` de origem.

Se você não está passando um valor para `urlId` na configuração do widget de comentários, então este será uma versão "limpa" da URL da página.

Você pode ver quais valores seus comentários têm para `URL ID` exportando-os.

### Preenchendo os campos "Para"

Para decidir para onde mover os comentários, precisamos saber o `URL ID` de destino e o `URL`.

O `URL ID` será o bucket que o comentário vai entrar. O campo `URL` é usado para que você possa navegar diretamente até o comentário a partir de e-mails e das ferramentas de moderação.

#### WordPress

Se você estiver usando WordPress, por exemplo, você inseriria os IDs dos artigos nos campos `URL ID` De/Para na ferramenta de migração, em vez de uma URL.

---