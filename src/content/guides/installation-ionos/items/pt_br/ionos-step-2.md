A seguir, vamos adicionar o código do widget FastComments ao seu site. Este código irá procurar por todos os formulários com o título `FastComments Goes Here` e
substituí-los pelo FastComments.

Então vamos em `Settings` no canto inferior esquerdo do editor do site:

<div class="screenshot white-bg">
    <div class="title">Abrir Configurações</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Abrir Configurações" />
</div>

Abra a seção `Custom Head Code`:

<div class="screenshot white-bg">
    <div class="title">Abrir Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Abrir Custom Head Code" />
</div>

Para o Ionos precisamos de uma **versão especial** do código do widget FastComments. Trechos de código de **outros tutoriais não funcionarão.**

Agora copie o código a seguir:

[inline-code-attrs-start title = 'Trecho do FastComments para Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // obtemos o elemento que não ocupa a largura total
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...e cole-o conforme mostrado:

<div class="screenshot white-bg">
    <div class="title">Colar e Salvar</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Colar e Salvar" />
</div>

---