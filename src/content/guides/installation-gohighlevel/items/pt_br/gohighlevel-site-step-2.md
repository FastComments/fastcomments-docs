É hora de copiar nosso código. Copie o código a seguir:

[inline-code-attrs-start title = 'Código de Comentários do Site GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const SCRIPT_ID = 'fastcomments-embed';
        const WIDGET_ID = 'fastcomments-widget';
    
        let lastInstance;
        let currentUrlId;
        let rendered = false;
    
        // Modificações na History API para suporte a SPA
        const oldPushState = history.pushState;
        history.pushState = function pushState() {
            const ret = oldPushState.apply(this, arguments);
            window.dispatchEvent(new Event('pushstate'));
            window.dispatchEvent(new Event('locationchange'));
            return ret;
        };
    
        const oldReplaceState = history.replaceState;
        history.replaceState = function replaceState() {
            const ret = oldReplaceState.apply(this, arguments);
            window.dispatchEvent(new Event('replacestate'));
            window.dispatchEvent(new Event('locationchange'));
            return ret;
        };
    
        window.addEventListener('popstate', () => {
            window.dispatchEvent(new Event('locationchange'));
        });
    
        function getContainer() {
            return document.getElementById(WIDGET_ID);
        }
    
        // Função para garantir que o script seja carregado
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Verifica se a tag de script já existe
                let scriptTag = document.getElementById(SCRIPT_ID);
    
                if (!scriptTag) {
                    console.log('FastComments: Script tag not found, adding dynamically...');
                    scriptTag = document.createElement('script');
                    scriptTag.id = SCRIPT_ID;
                    scriptTag.src = 'https://cdn.fastcomments.com/js/embed-v2.min.js';
                    scriptTag.async = true;
    
                    scriptTag.onload = () => {
                        console.log('FastComments: Script loaded successfully');
                        resolve();
                    };
    
                    scriptTag.onerror = () => {
                        console.error('FastComments: Failed to load script');
                        resolve(); // Resolve anyway to prevent hanging
                    };
    
                    document.head.appendChild(scriptTag);
                } else if (window.FastCommentsUI) {
                    // A tag do script existe e já foi carregada
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // A tag do script existe mas ainda não está pronta
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // Solução alternativa caso o script já esteja carregando
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // Tempo limite após 10 segundos
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // Função principal de renderização
        async function render() {
            rendered = false;
    
            // Ensure script is loaded before proceeding
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // Verifica novamente se FastCommentsUI está disponível
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // Obtém a URL atual como urlId
                    const newUrlId = window.location.pathname;
    
                    // Verifica se precisamos renderizar novamente (urlId mudou ou é a primeira renderização)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // Destrói a instância anterior, se existir
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Limpa o conteúdo do container
                            container.innerHTML = '';
                        }
    
                        // Prepara a configuração
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // Inicializa o FastComments
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // Monitora se o container for removido ou se a URL mudar
                    const interval = setInterval(function () {
                        const currentContainer = getContainer();
                        if (!currentContainer) {
                            console.log('FastComments: Container removed, will retry...');
                            rendered = false;
                            currentUrlId = null;
                            tryNext();
                            clearInterval(interval);
                        } else {
                            const newUrlId = window.location.pathname;
                            if (newUrlId !== currentUrlId) {
                                console.log('FastComments: URL changed, re-rendering...');
                                rendered = false;
                                tryNext();
                                clearInterval(interval);
                            }
                        }
                    }, 1000);
                } else {
                    console.log('FastComments: Target element not found, waiting...');
                    setTimeout(tryNext, 300);
                }
            }
    
            tryNext();
        }
    
        // Renderização inicial quando o DOM estiver pronto
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Re-renderiza ao mudar a localização (para SPAs)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

Cole isso na janela do editor que abrimos:

<div class="screenshot white-bg">
    <div class="title">Colar Código</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="Colar Código" />
</div>

Agora podemos clicar em `Yes, Save` no canto inferior direito dessa janela.

No topo da página agora clique em `Save` e depois em `Preview`.