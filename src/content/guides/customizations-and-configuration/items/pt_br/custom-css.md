[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments foi projetado para ser customizado. O widget de comentários roda dentro de um iframe por razões de segurança, então, para aplicar estilos personalizados, você deve seguir uma das duas abordagens.

A primeira, a abordagem mais fácil e a que preferimos, é usar a [página de personalização do widget](https://fastcomments.com/auth/my-account/customize-widget).

Na página de personalização do widget, veja a seção “Mostrar Opções Avançadas”, sob a qual há uma área rotulada como “Custom CSS”:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Editor de CSS personalizado sob Mostrar Opções Avançadas na página de personalização do widget'; title='Área de Entrada de CSS Personalizado' app-screenshot-end]

Esta abordagem tem alguns benefícios:
1. O CSS inserido é minificado antes de ser enviado ao usuário, e a formatação é mantida consistente na UI de edição.
2. Você obtém todos os benefícios da UI de personalização do widget, por exemplo, personalizando facilmente o widget de comentários de forma diferente para diferentes sites.
3. Quando fazemos alterações no widget de comentários, seu estilo personalizado será testado como parte do nosso processo de lançamento.

A segunda abordagem é especificar o parâmetro **customCSS** na configuração do widget, da seguinte forma:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Entretanto, isso tem *limitações*:
1. Existe um limite de quanto CSS personalizado pode ser passado antes que nossos servidores rejeitem a requisição, devido ao tamanho dos cabeçalhos.
2. Você deve gerenciar o CSS personalizado em sua infraestrutura e sistema de build. Isso pode ser uma vantagem em vez de uma desvantagem, também.
3. Há um custo adicional de enviar o CSS personalizado pela rede **duas vezes** neste caso de uso, pois ele precisa ser enviado aos nossos servidores e depois devolvido no conteúdo do iframe. Contudo, para a maioria dos tamanhos de payload, isso não é perceptível.
4. Uma otimização comum é minificar o CSS para reduzir seu tamanho na rede; porém, com esta abordagem, você terá que lidar com isso.
5. Seu CSS personalizado não será testado quando fizermos alterações.

### Arquivos CSS Externos

Você pode instruir o widget a buscar um arquivo externo usando `@import`!

É recomendado colocar o `@import` em uma regra de personalização. Dessa forma, se precisarmos fazer uma mudança no widget de comentários, podemos usar nossa automação
para verificar sua configuração. Por exemplo, você criaria uma regra de personalização na UI de Personalização do Widget, clicaria em `Avançado` e inseriria em `Custom CSS`:

    @import url(https://example.com/styles.css);

#### No Código - Não Recomendado

Você também pode carregar um arquivo CSS externo via a propriedade `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Entretanto, lembre‑se de que seu CSS não poderá ser testado por nós se você fizer isso. 

### Estilização do Modal de Perfil de Usuário

Modais de perfil de usuário também podem ser estilizados com CSS personalizado. Contudo, para garantir que o estilo personalizado seja aplicado aos perfis de usuário, todos os seletores CSS devem ser prefixados com `.user-profile`. Sem esse prefixo, o estilo personalizado será ignorado nos modais de perfil de usuário.

Por exemplo:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Compatibilidade Retroativa

Na FastComments, sabemos que nossos clientes personalizam o widget de comentários. Isso faz parte do design – a última coisa que queremos é que nosso produto cause inconsistências de design em seu produto.

Como isso é uma parte importante do nosso produto, temos um pipeline de build que nos permite revisar mudanças no widget de comentários, por cliente, a cada lançamento.

Se encontrarmos pequenos problemas, atualizaremos sua conta para garantir que nosso lançamento ocorra sem contratempos. Se identificarmos mudanças significativas que quebrem a compatibilidade, isso nos permite interromper o lançamento.