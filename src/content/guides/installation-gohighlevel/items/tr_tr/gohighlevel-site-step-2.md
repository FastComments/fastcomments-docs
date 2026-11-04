Kodumuzu kopyalama zamanı. Aşağıdaki kodu kopyalayın:

[inline-code-attrs-start title = 'GoHighLevel Site Yorumları Kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // SPA desteği için History API değişiklikleri
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
    
        // Betiğin yüklendiğinden emin olmak için fonksiyon
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Script etiketinin zaten var olup olmadığını kontrol et
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
                        resolve(); // Takılmayı önlemek için yine de resolve et
                    };
    
                    document.head.appendChild(scriptTag);
                } else if (window.FastCommentsUI) {
                    // Script etiketi mevcut ve zaten yüklü
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // Script etiketi mevcut ama henüz hazır değil
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // Betik zaten yükleniyorsa ihtiyaç halinde yedek çözüm
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // 10 saniye sonra zaman aşımı
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // Ana render fonksiyonu
        async function render() {
            rendered = false;
    
            // Devam etmeden önce betiğin yüklendiğinden emin ol
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // FastCommentsUI'in kullanılabilir olup olmadığını tekrar kontrol et
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // Mevcut URL'i urlId olarak al
                    const newUrlId = window.location.pathname;
    
                    // Yeniden render etmemiz gerekip gerekmediğini kontrol et (urlId değişti veya ilk render)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // Varsa önceki örneği yok et
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Konteyner içeriğini temizle
                            container.innerHTML = '';
                        }
    
                        // Yapılandırmayı hazırla
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // FastComments'i başlat
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // Konteynerin kaldırılıp kaldırılmadığını veya URL'in değişip değişmediğini izle
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
    
        // DOM hazır olduğunda ilk render
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Konum değişikliğinde yeniden render et (SPA'lar için)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

Bunu, açtığımız editör penceresine yapıştırın:

<div class="screenshot white-bg">
    <div class="title">Kodu Yapıştır</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="Kodu Yapıştır" />
</div>

Artık o pencerenin sağ alt kısmındaki `Yes, Save` düğmesine tıklayabiliriz.

Sayfanın en üstünde şimdi `Save` ve ardından `Preview` düğmelerine tıklayın.