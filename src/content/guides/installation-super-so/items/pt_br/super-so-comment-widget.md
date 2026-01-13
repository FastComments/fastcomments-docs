## Adicionando um widget de comentários ao vivo aos seus artigos do Notion no Super.so

Além do Collab Chat, você pode adicionar um widget de comentários tradicional ao final dos seus artigos do Notion. Isso permite que os leitores deixem comentários e conversem sobre o artigo inteiro.

### Etapas de instalação

Copie o código a seguir e cole-o na seção `Body` nas configurações do seu site no Super.so:

[inline-code-attrs-start title = 'Widget de Comentários ao Vivo do FastComments para Super.so'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Limpar instância existente
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Criar novo alvo
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Inicializar o FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Atualizar pathname atual
            currentPathname = window.location.pathname;
        }

        // Carregamento inicial
        load();

        // Verificar alterações a cada 500ms
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
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Observações importantes

- O widget de comentários aparecerá na parte inferior dos seus artigos do Notion
- Cada página obtém seu próprio tópico de comentários exclusivo com base no caminho da URL
- Certifique-se de substituir `"demo"` pelo seu tenant ID real da sua conta FastComments
- O widget lida automaticamente com o carregamento dinâmico de páginas do Super.so

---