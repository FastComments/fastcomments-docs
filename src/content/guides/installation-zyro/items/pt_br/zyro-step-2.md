Agora vamos adicionar o código do nosso widget.

Copie o código abaixo. Você deve garantir que está conectado em [fastcomments.com](https://fastcomments.com) 
e recarregar esta página se não estiver, para que o código seja pré-preenchido com as informações da sua conta; caso contrário, ele exibirá o código de demonstração.

Agora vamos copiar o código:

[inline-code-attrs-start title = 'Código de Comentários do Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Agora volte para o construtor do seu site e clique em `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Inserir código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Inserir código" />
</div>

### Observação!

É importante que você use o código acima e não os trechos de código de outras documentações, pois este trecho foi elaborado especificamente
para o Zyro.

Você agora deve ter algo semelhante ao seguinte, que aparece em branco. Isso é esperado. Passe o mouse sobre a área
onde o widget deve estar:

<div class="screenshot white-bg">
    <div class="title">Widget de Código Adicionado</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de Código Adicionado" />
</div>

Agora arraste o widget até o tamanho desejado; você verá que ele aparece:

<div class="screenshot white-bg">
    <div class="title">Redimensione</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Redimensione" />
</div>

...e agora visualize e salve!