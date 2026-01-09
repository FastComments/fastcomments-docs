No próximo passo você precisa copiar o código do widget pré-criado abaixo.

Contanto que você esteja logado em FastComments.com o trecho de código abaixo já terá as informações da sua conta. Vamos copiá-lo:

[inline-code-attrs-start title = 'Código do Super.so FastComments Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;
        let currentTopBar = null;

        function load() {
            if (!window.FastCommentsCollabChat) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const target = document.querySelector('.super-content');
            if (!target || !target.innerHTML || target.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Limpar instância existente
            if (target.fastCommentsInstance) {
                target.fastCommentsInstance.destroy();
            }

            // Remover top bar existente, se houver
            if (currentTopBar && currentTopBar.parentNode) {
                currentTopBar.parentNode.removeChild(currentTopBar);
            }

            // Criar nova top bar
            const topBarTarget = document.createElement('div');
            target.parentNode.insertBefore(topBarTarget, target);
            topBarTarget.style.maxWidth = 'var(--layout-max-width)';
            topBarTarget.style.margin = '0 auto';
            currentTopBar = topBarTarget;
            currentWidget = target;

            // Inicializar FastComments Collab Chat
            target.fastCommentsInstance = FastCommentsCollabChat(target, {
                tenantId: "demo",
                topBarTarget: topBarTarget
            });

            // Atualizar pathname atual
            currentPathname = window.location.pathname;
        }

        // Carregamento inicial
        load();

        // Verificar mudanças a cada 500ms
        setInterval(() => {
            // Recarregar se o pathname mudou
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Recarregar se o widget foi removido
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Recarregar se o container foi esvaziado
            const target = document.querySelector('.super-content');
            if (target && target.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

Agora cole na área `Body`:

<div class="screenshot white-bg">
    <div class="title">Código colado</div>
    <img class="screenshot-image" src="/images/installation-guides/super-so-step-3-pasted-code.png" alt="Código colado" />
</div>

Se você vir a mensagem "esta é uma mensagem de demonstração" após colar o código:

- Certifique-se de que você está logado na sua conta em fastcomments.com.
- Certifique-se de que cookies de terceiros estejam habilitados.
- Em seguida, atualize esta página e copie o trecho de código novamente. Ele deverá ter `tenantId` preenchido com o identificador do seu tenant.