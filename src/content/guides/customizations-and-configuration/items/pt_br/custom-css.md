[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments foi projetado para ser customizado. O próprio widget de comentários roda dentro de um iframe por razões de segurança, então para aplicar
estilos personalizados você precisa seguir uma de duas abordagens.

A primeira, a mais fácil, e a preferida por nós, é usar a [página de personalização do widget](https://fastcomments.com/auth/my-account/customize-widget).

Na página de personalização do widget, veja a seção "Show Advanced Options", sob a qual há uma área rotulada "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Esta abordagem tem alguns benefícios:
1. O CSS inserido é minificado antes de ser enviado ao usuário, e a formatação é mantida consistente na interface de edição.
2. Você obtém todos os benefícios da interface de personalização do widget, por exemplo personalizar facilmente o widget de comentários de forma diferente para sites diferentes.
3. Quando fazemos alterações no widget de comentários, seu estilo customizado será testado como parte do nosso processo de liberação.

A segunda abordagem é especificar o parâmetro **customCSS** na configuração do widget, como segue:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

No entanto, isso tem *limitações*:
1. Há um limite de quanto CSS personalizado pode ser enviado antes que nossos servidores rejeitem a requisição, devido ao tamanho dos cabeçalhos.
2. Você deve gerenciar o CSS personalizado na sua infraestrutura e sistema de build. Isso pode ser uma vantagem em vez de uma desvantagem, também.
3. Há um overhead adicional de enviar o CSS personalizado pela rede **duas vezes** neste caso de uso, já que ele precisa ser enviado aos nossos servidores e então enviado de volta no conteúdo do iframe. Contudo, para a maioria dos tamanhos de payload, isso não é perceptível.
4. Uma otimização comum é minificar o CSS para reduzir seu tamanho na rede, porém com esta abordagem você terá que lidar com isso.
5. Seu CSS customizado não será testado quando fizermos alterações.

### External CSS Files

Você pode instruir o widget a buscar um arquivo externo usando `@import`!

É recomendado colocar o `@import` em uma regra de customização. Desta forma, se algum dia precisarmos fazer uma alteração no widget de comentários, podemos usar nossas ferramentas de automação
para verificar sua configuração. Então, por exemplo, você criaria uma regra de customização na UI de Personalização do Widget, clicaria em `Advanced`, e entraria em `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Você também pode carregar um arquivo CSS externo via a propriedade `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

No entanto, lembre-se que seu CSS não poderá ser testado por nós se você fizer isso. 

### User Profile Modal Styling

Os modais de perfil do usuário também podem ser estilizados com CSS customizado. Porém, para garantir que o estilo personalizado seja aplicado aos perfis de usuário, todos os seletores CSS devem ser prefixados com `.user-profile`. Sem esse prefixo, o estilo personalizado será ignorado para os modais de perfil do usuário.

Por exemplo:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

Na FastComments, sabemos que nossos clientes personalizam o widget de comentários. Isso é intencional — a última coisa que queremos é que nosso produto cause inconsistências de design no seu produto.

Como esta é uma parte importante do nosso produto, temos um pipeline de build que nos permite revisar alterações no widget de comentários, por cliente, a cada release.

Se encontrarmos problemas menores, atualizaremos sua conta para garantir que nossa liberação aconteça sem problemas. Se virmos mudanças significativas que quebrem, isso nos permite interromper a liberação.

---