Вријеме је да копирате наш код. Копирајте следећи код:

[inline-code-attrs-start title = 'GoHighLevel код коментара за сајт'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // Измјене History API за подршку SPA
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
    
        // Функција за осигурање да је скрипта учитана
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Провјерите да ли таг скрипте већ постоји
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
                        resolve(); // Свеједно решити да се спречи заглављивање
                    };
    
                    document.head.appendChild(scriptTag);
                } else if (window.FastCommentsUI) {
                    // Таг скрипте постоји и је већ учитан
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // Таг скрипте постоји али још није иницијализован
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // Резервна провјера у случају да се скрипта већ учитава
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // Истек времена након 10 секунди
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // Главна функција за рендеровање
        async function render() {
            rendered = false;
    
            // Осигурајте да је скрипта учитана прије наставка
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // Додатна провјера да ли је FastCommentsUI доступан
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // Узми тренутни URL као urlId
                    const newUrlId = window.location.pathname;
    
                    // Провјери да ли треба поново рендеровати (urlId је промијењен или је ово прво рендеровање)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // Уништи претходну инстанцу ако постоји
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Очисти садржај контејнера
                            container.innerHTML = '';
                        }
    
                        // Припреми конфигурацију
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // Иницијализуј FastComments
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // Праћење ако се контејнер уклони или ако се URL промијени
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
    
        // Почетно рендеровање када је DOM спреман
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Поновно рендеровање при промјени локације (за SPA)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

Paste that in the editor window we opened:

<div class="screenshot white-bg">
    <div class="title">Залепите код</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="Залепите код" />
</div>

Сада можемо кликнути на `Yes, Save` у доњем десном углу тог прозора.

На врху странице сада кликните `Save` и затим `Preview`.