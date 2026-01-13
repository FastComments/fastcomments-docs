Agora vamos adicionar o código do nosso widget.

Copie o código abaixo. Você deve garantir que está conectado em [fastcomments.com](https://fastcomments.com) 
e recarregar esta página se não estiver, para que o código seja preenchido automaticamente com as informações da sua conta, caso contrário ele mostrará o código de demonstração.

Agora vamos copiar o código:

[inline-code-attrs-start title = 'Código de comentários do Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Agora vamos voltar ao construtor do site e clicar em `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Inserir código</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Inserir código" />
</div>

### Note!

É importante que você use o código acima e não os trechos de código de outra documentação, pois este trecho foi criado especificamente
para Hostinger.

Você deve agora ter algo parecido com o seguinte, que aparece em branco. Isso é esperado. Mova o mouse sobre a área
onde o widget deve estar:

<div class="screenshot white-bg">
    <div class="title">Widget de código adicionado</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de código adicionado" />
</div>

Agora arraste o widget para o tamanho desejado, você verá que ele aparecerá:

<div class="screenshot white-bg">
    <div class="title">Redimensione</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Redimensione" />
</div>

...e agora visualize e salve!

---