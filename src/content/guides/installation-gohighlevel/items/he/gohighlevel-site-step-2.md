הגיע הזמן להעתיק את הקוד שלנו. העתק את הקוד הבא:

[inline-code-attrs-start title = 'קוד תגובות לאתר GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // שינויים ב-History API לתמיכה ב-SPA
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
    
        // פונקציה להבטחת טעינת הסקריפט
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // בדוק אם תג הסקריפט כבר קיים
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
                    // תג הסקריפט קיים וכבר נטען
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // תג הסקריפט קיים אך עדיין לא מוכן
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // פתרון חלופי במקרה שהסקריפט כבר בטעינה
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // פסק זמן לאחר 10 שניות
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // פונקציית רינדור ראשית
        async function render() {
            rendered = false;
    
            // ודא שהסקריפט נטען לפני שממשיכים
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // בדיקה כפולה אם FastCommentsUI זמין
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // קבל את ה-URL הנוכחי כ-urlId
                    const newUrlId = window.location.pathname;
    
                    // בדוק אם יש צורך ברינדור מחדש (urlId השתנה או רינדור ראשון)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // השמד מופע קודם אם קיים
                        if (lastInstance) {
                            lastInstance.destroy();
                            // נקו את תוכן המיכל
                            container.innerHTML = '';
                        }
    
                        // הכן את הקונפיגורציה
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // אתחל את FastComments
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // מעקב האם המיכל הוסר או שה-URL משתנה
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
    
        // Initial render when DOM is ready
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Re-render on location change (for SPAs)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

הדבק את זה בחלון העורך שפתחנו:

<div class="screenshot white-bg">
    <div class="title">הדבק קוד</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="הדבק קוד" />
</div>

כעת נוכל ללחוץ על `Yes, Save` בפינה הימנית התחתונה של אותו חלון.

בחלק העליון של הדף, לחצו עכשיו על `Save` ואז על `Preview`.