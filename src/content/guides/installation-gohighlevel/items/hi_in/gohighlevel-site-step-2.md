अब हमारा कोड कॉपी करने का समय है। निम्नलिखित कोड कॉपी करें:

[inline-code-attrs-start title = 'GoHighLevel साइट कमेंट्स कोड'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // SPA सपोर्ट के लिए History API में बदलाव
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
    
        // स्क्रिप्ट लोड होना सुनिश्चित करने का फंक्शन
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // जाँचें कि क्या स्क्रिप्ट टैग पहले से मौजूद है
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
                        resolve(); // हैंगिंग रोकने के लिए फिर भी resolve करें
                    };
    
                    document.head.appendChild(scriptTag);
                } else if (window.FastCommentsUI) {
                    // स्क्रिप्ट टैग मौजूद है और पहले से लोड हो चुका है
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // स्क्रिप्ट टैग मौजूद है लेकिन अभी तैयार नहीं है
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // फॉलबैक, अगर स्क्रिप्ट पहले से लोड हो रही हो
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // 10 सेकंड बाद टाइमआउट
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // मुख्य रेंडर फंक्शन
        async function render() {
            rendered = false;
    
            // आगे बढ़ने से पहले सुनिश्चित करें कि स्क्रिप्ट लोड हो चुकी है
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // दोबारा जाँचें कि FastCommentsUI उपलब्ध है या नहीं
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // urlId के रूप में वर्तमान URL लें
                    const newUrlId = window.location.pathname;
    
                    // जाँचें कि क्या हमें दोबारा रेंडर करना है (urlId बदला है या पहला रेंडर है)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // यदि पिछला इंस्टेंस मौजूद है तो उसे नष्ट करें
                        if (lastInstance) {
                            lastInstance.destroy();
                            // कंटेनर की सामग्री साफ़ करें
                            container.innerHTML = '';
                        }
    
                        // कॉन्फ़िग तैयार करें
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // FastComments इनिशियलाइज़ करें
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // देखें कि कंटेनर हटा दिया गया है या URL बदल गया है
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
    
        // DOM तैयार होने पर प्रारंभिक रेंडर
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // लोकेशन बदलने पर दोबारा रेंडर (SPA के लिए)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

इसे उस एडिटर विंडो में पेस्ट करें जो हमने खोली थी:

<div class="screenshot white-bg">
    <div class="title">कोड पेस्ट करें</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="कोड पेस्ट करें" />
</div>

अब हम उस विंडो के निचले दाएँ कोने पर `Yes, Save` पर क्लिक कर सकते हैं।

अब पेज के शीर्ष पर `Save` पर क्लिक करें और फिर `Preview` पर।