Aqui estão alguns sintomas que encontramos com frequência, e soluções comuns.

### Mensagem "This is a demo"

Isso é exibido quando você copiou o código do widget da nossa página inicial, que usa nosso tenant de demonstração. Para usar o seu tenant, copie o código do widget [daqui](https://fastcomments.com/auth/my-account/get-acct-code).

### Erro "FastComments cannot load on this domain"

O FastComments precisa saber quais domínios são de sua propriedade para autenticar requisições associadas à sua conta. [Confira nossa documentação](/guide-multiple-sites.html#add-domains-to-account) para ver como resolver esse erro (basta adicionar o subdomínio + domínio exatos à sua conta).

Observe que isso só deve ocorrer após o fim do período de teste. Durante o período de teste, quaisquer requisições de novos domínios serão adicionadas automaticamente à sua conta.

### Comentários Migrados Não Aparecendo em Instalações Personalizadas

Normalmente isso acontece quando os comentários importados estão vinculados a um `Page ID`, e você está passando uma URL (ou nenhum valor, caso em que o padrão é a URL da página).

Você pode depurar isso [exportando seus comentários](https://fastcomments.com/auth/my-account/manage-data/export) e visualizando a coluna `URL ID` (atualmente a coluna `B`).

Certifique-se de que os valores que você vê na coluna `URL ID` são os mesmos valores que você está passando para a configuração do widget como o parâmetro `urlId`.

Para uma explicação mais detalhada, tente ler nossa documentação [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Se nada disso funcionar, [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Widget de Comentários Não Aparecendo

Se o widget de comentários não estiver aparecendo, verifique o console de desenvolvedor do Chrome em busca de erros.

Para a maioria das configurações incorretas, o widget de comentários mostrará pelo menos um erro na página se conseguir carregar. Não ver nada geralmente indica um erro de script.

### Configuração Desejada Não Funcionando Como Esperado

Experimente nossa [extensão do Chrome](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) para ver qual configuração está sendo passada ao widget de comentários. Se nada funcionar, tire um screenshot do que a extensão do Chrome mostra e [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Comentários Ausentes na Mesma URL com Hash Bang Diferente

Por padrão, o FastComments usará a URL da página como o "bucket" onde os comentários são armazenados. Se suas URLs incluem `#hashbangs`, e esses `#hashbangs` não devem fazer parte do identificador que identifica uma thread de comentários, podemos simplesmente ignorar o valor do hash bang, por exemplo:

[inline-code-attrs-start title = 'Exemplo de Ignorar Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Observe que, após fazer essa alteração, uma migração terá que ser realizada para os comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### Parâmetros de Query da URL Afetando o Widget

Por padrão, o FastComments usará a URL da página como o "bucket" onde os comentários são armazenados. Se suas URLs incluem parâmetros de query que não devem fazer parte do identificador que identifica uma thread de comentários, podemos simplesmente ignorá-los, por exemplo:

[inline-code-attrs-start title = 'Ignorar Parâmetros de Query'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Observe que, após fazer essa alteração, uma migração terá que ser realizada para os comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### Não Estou Recebendo E-mails

Na FastComments, dedicamos muito esforço para garantir que a entrega dos nossos e-mails seja a mais confiável possível. No entanto, alguns provedores de e-mail são notoriamente difíceis para entregas confiáveis. Verifique sua pasta de spam em busca de mensagens de fastcomments.com.

Se você [entrar em contato conosco](https://fastcomments.com/auth/my-account/help), geralmente podemos fornecer mais informações sobre por que você pode não estar vendo e-mails nossos.