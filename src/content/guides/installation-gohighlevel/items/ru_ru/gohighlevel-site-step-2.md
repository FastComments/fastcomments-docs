Пора скопировать наш код. Скопируйте следующий код:

[inline-code-attrs-start title = 'Код комментариев сайта GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // Модификации History API для поддержки SPA
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
    
        // Функция для проверки загрузки скрипта
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Проверяем, существует ли уже тег script
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
                    // Тег script существует и уже загружен
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // Тег script существует, но еще не готов
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // Запасной вариант на случай, если скрипт уже загружается
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // Таймаут через 10 секунд
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // Основная функция рендера
        async function render() {
            rendered = false;
    
            // Убедиться, что скрипт загружен перед продолжением
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // Дополнительная проверка на наличие FastCommentsUI
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // Получаем текущий URL как urlId
                    const newUrlId = window.location.pathname;
    
                    // Проверяем, нужно ли перерисовывать (urlId изменился или первый рендер)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // Уничтожаем предыдущий экземпляр, если он есть
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Очищаем содержимое контейнера
                            container.innerHTML = '';
                        }
    
                        // Готовим конфигурацию
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // Инициализируем FastComments
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // Отслеживаем удаление контейнера или изменение URL
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
    
        // Первичный рендер при готовности DOM
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Перерендер при изменении локации (для SPA)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

Вставьте это в окно редактора, которое мы открыли:

<div class="screenshot white-bg">
    <div class="title">Вставьте код</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="Вставьте код" />
</div>

Теперь мы можем нажать `Yes, Save` в правом нижнем углу этого окна.

Теперь в верхней части страницы нажмите `Save`, а затем `Preview`.

---