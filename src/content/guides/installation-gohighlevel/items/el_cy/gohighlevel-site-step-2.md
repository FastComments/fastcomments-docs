Είναι ώρα να αντιγράψουμε τον κώδικά μας. Αντιγράψτε τον παρακάτω κώδικα:

[inline-code-attrs-start title = 'Κώδικας σχολίων ιστότοπου GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    
        // Τροποποιήσεις του History API για υποστήριξη SPA
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
    
        // Συνάρτηση για να διασφαλιστεί ότι το script έχει φορτωθεί
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Έλεγχος αν το tag script υπάρχει ήδη
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
                    // Το tag script υπάρχει και έχει ήδη φορτωθεί
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // Το tag script υπάρχει αλλά δεν είναι ακόμα έτοιμο
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });
    
                    // Εναλλακτική μέθοδος σε περίπτωση που το script ήδη φορτώνεται
                    const checkInterval = setInterval(() => {
                        if (window.FastCommentsUI) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);
    
                    // Χρονικό όριο μετά από 10 δευτερόλεπτα
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }
    
        // Κύρια συνάρτηση render
        async function render() {
            rendered = false;
    
            // Διασφαλίστε ότι το script έχει φορτωθεί πριν προχωρήσετε
            await ensureScriptLoaded();
    
            function tryNext() {
                if (rendered) {
                    return;
                }
    
                const container = getContainer();
    
                if (container) {
                    // Επαλήθευση ότι το FastCommentsUI είναι διαθέσιμο
                    if (!window.FastCommentsUI) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
    
                    console.log('FastComments: Target element found, initializing...');
    
                    // Λήψη του τρέχοντος URL ως urlId
                    const newUrlId = window.location.pathname;
    
                    // Έλεγχος αν χρειάζεται επαναπροσδιορισμός (αλλαγή urlId ή πρώτη απόδοση)
                    if (currentUrlId !== newUrlId || !lastInstance) {
                        currentUrlId = newUrlId;
    
                        // Καταστροφή της προηγούμενης παρουσίας αν υπάρχει
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Εκκαθάριση του περιεχομένου του container
                            container.innerHTML = '';
                        }
    
                        // Προετοιμασία της διαμόρφωσης
                        const config = {
                            tenantId: tenantId,
                            urlId: newUrlId
                        };
    
                        console.log('FastComments: Using urlId:', newUrlId);
    
                        // Αρχικοποίηση του FastComments
                        lastInstance = window.FastCommentsUI(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }
    
                    // Παρακολούθηση αν το container αφαιρεθεί ή αλλάξει το URL
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
    
        // Αρχική απόδοση όταν το DOM είναι έτοιμο
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', render);
        } else {
            render();
        }
    
        // Επανα-απόδοση σε αλλαγή τοποθεσίας (για SPAs)
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

Επικολλήστε το στο παράθυρο του επεξεργαστή που ανοίξαμε:

<div class="screenshot white-bg">
    <div class="title">Επικόλληση Κώδικα</div>
    <img class="screenshot-image" src="/images/installation-guides/gohighlevel-site-step-7-paste-code.png" alt="Επικόλληση Κώδικα" />
</div>

Τώρα μπορούμε να κάνουμε κλικ στο `Yes, Save` στο κάτω δεξιά μέρος αυτού του παραθύρου.

Στην κορυφή της σελίδας κάντε τώρα κλικ στο `Save` και στη συνέχεια στο `Preview`.