Aqui estão alguns sintomas que encontramos frequentemente e soluções comuns.

### Mensagem "This is a demo"

Isso é mostrado quando você copiou o código do widget da nossa página inicial, que usa nosso tenant de demonstração. Para usar seu tenant, copie o código do widget de [aqui](https://fastcomments.com/auth/my-account/get-acct-code).

### Erro "FastComments cannot load on this domain"

FastComments precisa saber quais domínios pertencem a você para autenticar requisições associadas à sua conta. [Confira nossa documentação](/guide-multiple-sites.html#add-domains-to-account) para ver como resolver este erro (simplesmente adicione o subdomínio exato + domínio à sua conta).

Note que isso só deve ocorrer após o período de teste terminar. Durante o período de teste, quaisquer requisições de novos domínios serão automaticamente adicionadas à sua conta.

### Comentários Migrados Não Aparecem para Instalações Personalizadas

Geralmente isso acontece quando os comentários importados estão vinculados a um `Page ID`, e você está passando uma URL (ou nenhum valor, neste caso usa por padrão a URL da página).

Você pode depurar isso [exportando seus comentários](https://fastcomments.com/auth/my-account/manage-data/export) e visualizando a coluna `URL ID` (atualmente Coluna `B`).

Certifique-se de que os valores que você vê na coluna `URL ID` são os mesmos valores que você está passando para a configuração do widget como parâmetro `urlId`.

Para mais explicações, tente ler nossa [documentação sobre Como Comentários são Vinculados a Páginas e Artigos](/guide-customizations-and-configuration.html#url-id).

Se tudo mais falhar, [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Widget de Comentários Não Aparece

Se o widget de comentários não está aparecendo, verifique o console de desenvolvedor do Chrome por erros.

Para a maioria das configurações incorretas, o widget de comentários pelo menos mostrará um erro na página se conseguir carregar. Não ver nada geralmente é uma indicação de um erro de script.

### Configuração Desejada Não Funciona Como Esperado

Experimente nossa [extensão Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) para ver qual configuração o widget de comentários está recebendo. Se tudo falhar, tire um screenshot do que a extensão Chrome mostra e [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Comentários Faltando na Mesma URL com Hash Bang Diferente

Por padrão, FastComments usará a URL da página para o "bucket" onde os comentários são armazenados. Se suas URLs incluem `#hashbangs`, e estes `#hashbangs` não devem fazer parte do identificador que identifica uma thread de comentários, podemos simplesmente ignorar o valor do hash bang, por exemplo:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Note que após fazer esta mudança, uma migração terá que ser realizada para comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### Parâmetros de Query da URL Afetando o Widget

Por padrão, FastComments usará a URL da página para o "bucket" onde os comentários são armazenados. Se suas URLs incluem parâmetros de query que não devem fazer parte do identificador que identifica uma thread de comentários, podemos simplesmente ignorá-los, por exemplo:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Note que após fazer esta mudança, uma migração terá que ser realizada para comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### Não Recebendo Emails

No FastComments, colocamos muito trabalho para garantir que nossa entrega de emails seja o mais confiável possível. No entanto, alguns provedores de email são notoriamente difíceis de alcançar de forma confiável. Verifique sua pasta de spam por mensagens de fastcomments.com.

Se você [entrar em contato conosco](https://fastcomments.com/auth/my-account/help) geralmente podemos fornecer mais informações sobre por que você pode não estar vendo emails nossos.
