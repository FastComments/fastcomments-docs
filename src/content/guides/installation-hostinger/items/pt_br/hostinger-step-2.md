Agora vamos adicionar o código do nosso widget.

Copie o código abaixo. Certifique-se de que você está conectado em [fastcomments.com](https://fastcomments.com) 
e recarregue esta página se não estiver, para que o código seja pré-preenchido com as informações da sua conta; caso contrário ele mostrará o código de demonstração.

Agora copie o código:

[inline-code-attrs-start title = 'Código de comentários Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Agora volte ao construtor do site e clique em `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Inserir código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Inserir código" />
</div>

### Nota!

É importante que você use o código acima e não os trechos de código de outra documentação, pois este snippet foi elaborado especificamente
para o Hostinger.

Agora você deve ter algo parecido com o seguinte, que parece em branco. Isso é esperado. Passe o mouse sobre a área
onde o widget deve aparecer:

<div class="screenshot white-bg">
    <div class="title">Widget de código adicionado</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de código adicionado" />
</div>

Agora arraste o widget para o tamanho desejado; você verá ele aparecer:

<div class="screenshot white-bg">
    <div class="title">Redimensionar</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Redimensionar" />
</div>

...e agora pré-visualize e salve!