Na seção **Rodapé** da aba Código Personalizado, cole o código a seguir:

[inline-code-attrs-start title = 'Trecho de Código de Comentários ao Vivo do Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Cole o código na seção Rodapé</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Cole o código do FastComments na seção Rodapé" />
</div>

Após colar o código, clique no botão **Salvar** para aplicar suas alterações.

Observação: este código inclui lógica para posicionar dinamicamente o widget do FastComments no local mais adequado das postagens do seu blog Typeflo.io. Outros trechos de código não funcionarão corretamente com o layout do Typeflo.io.

Lembre-se de substituir 'demo' pelo seu ID de tenant do FastComments após se inscrever. Se você estiver logado no FastComments.com, isso já deverá ter sido substituído.