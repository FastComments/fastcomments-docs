Aqui estão alguns sintomas que encontramos com frequência e soluções comuns. 

### "This is a demo" Message

Isto é exibido quando você copiou o código do widget da nossa página inicial, que usa nosso tenant de demonstração. Para usar seu tenant, copie o código do widget a partir [daqui](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Error

O FastComments precisa saber quais domínios são de sua propriedade para autenticar requisições associadas à sua conta. [Consulte nossa documentação](/guide-multiple-sites.html#add-domains-to-account) para ver como resolver este erro (simplesmente adicione o subdomínio + domínio exatos à sua conta).

Observe que isso só deve ocorrer após o término do período de avaliação. Durante o período de avaliação, quaisquer requisições de novos domínios serão adicionadas automaticamente à sua conta.

### Migrated Comments Not Showing for Custom Installations

Normalmente isso acontece quando os comentários importados estão vinculados a um `Page ID`, e você está passando uma URL (ou nenhum valor, caso em que ela padrão para a URL da página).

Você pode depurar isso [exportando seus comentários](https://fastcomments.com/auth/my-account/manage-data/export) e visualizando a coluna `URL ID` (atualmente Coluna `B`).

Certifique-se de que os valores que você vê na coluna `URL ID` sejam os mesmos valores que você está passando para a configuração do widget como o parâmetro `urlId`.

Para mais explicações, tente ler nossa documentação [How Comments are Tied to Pages and Articles](/guide-customizations-and-configuration.html#url-id).

Se tudo mais falhar, [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Se o widget de comentários não estiver aparecendo, verifique o console de desenvolvedor do Chrome em busca de erros.

Para a maioria das configurações incorretas, o widget de comentários ao menos exibirá um erro na página se conseguir carregar. Não ver nada geralmente indica um erro de script.

### Desired Configuration Not Working as Expected

Tente nossa [extensão do Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) para ver qual configuração está sendo passada ao widget de comentários. Se tudo falhar, tire uma captura de tela do que a extensão do Chrome mostra e [entre em contato conosco](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Por padrão, o FastComments usará a URL da página para o "bucket" onde os comentários são armazenados. Se suas URLs incluem `#hashbangs`, e esses `#hashbangs`
não devem fazer parte do identificador que identifica um tópico de comentários, podemos simplesmente ignorar o valor do hash bang, por exemplo:

[inline-code-attrs-start title = 'Exemplo: Ignorar Hash Bangs'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Observe que depois de fazer essa alteração, uma migração terá que ser realizada para os comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Por padrão, o FastComments usará a URL da página para o "bucket" onde os comentários são armazenados. Se suas URLs incluem parâmetros de consulta
que não devem fazer parte do identificador que identifica um tópico de comentários, podemos simplesmente ignorá-los, por exemplo:

[inline-code-attrs-start title = 'Ignorar Parâmetros de Consulta'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Observe que depois de fazer essa alteração, uma migração terá que ser realizada para os comentários existentes. [Para isso, entre em contato conosco.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

No FastComments, nós nos esforçamos bastante para garantir que a entrega dos nossos e-mails seja o mais confiável possível. Contudo, alguns provedores de e-mail são notoriamente difíceis de entregar de forma confiável. Verifique sua pasta de spam por mensagens de fastcomments.com.

Se você [entrar em contato conosco](https://fastcomments.com/auth/my-account/help) geralmente podemos fornecer mais informações sobre por que você pode não estar recebendo e-mails nossos.

---