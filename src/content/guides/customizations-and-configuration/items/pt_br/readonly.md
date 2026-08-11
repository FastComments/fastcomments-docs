[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

A postagem de comentários pode ser bloqueada para que novos comentários ou votos não possam ser deixados definindo a flag readonly como true.

Os comentários também não poderão ser editados ou excluídos.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Tornando o Thread de Comentários Somente Leitura'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget, para um domínio inteiro ou página:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Configuração que impede novas respostas na página de personalização do widget, que bloqueia um thread para um domínio ou página'; title='Tornando o Thread de Comentários Somente Leitura' app-screenshot-end]

## Update!

A partir de novembro de 2022, os threads podem ser bloqueados ou desbloqueados **ao vivo** por administradores e moderadores via o menu de três pontos acima da área de resposta.

Isso impedirá novos comentários, enquanto ainda permite votação e permite que os usuários excluam seus comentários, se desejado, enquanto `readonly` não permite essas coisas. 

Isso corresponde ao campo `isClosed` na API `Page`.