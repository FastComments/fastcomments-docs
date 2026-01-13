[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Os comentários podem ser bloqueados para que nenhum novo comentário ou voto possa ser feito definindo a flag readonly como true.

Os comentários também não poderão ser editados ou excluídos.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget, para um domínio inteiro ou página:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Atualização!

A partir de novembro de 2022, os tópicos podem ser travados ou destravados **ao vivo** por administradores e moderadores através do menu de três pontos acima da área de resposta.

Isso impedirá novos comentários, ao mesmo tempo em que ainda permite votação e que os usuários excluam seus comentários, se desejarem, enquanto `readonly` não permite essas ações. 

Isso corresponde ao campo `isClosed` na API `Page`.

---