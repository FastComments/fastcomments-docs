Now we're going to copy our FastComments code. Try using the "Copy" button on the right, as the code is quite large to work properly
with GoHighLevel:

[inline-code-attrs-start title = 'GoHighLevel FastComments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script>
  <script>
    (function () {
        const tenantId = 'demo';
        const VALID_PATTERNS = ['/post'];
        const TYPE = 'commenting'; // set to one of: commenting, live, collab
        const SCRIPT_ID = 'fastcomments-embed';
        const TARGET_ELEMENT_ID = ''; // you can set this to add the widget to a specific area of the page by adding a separate HTML element with a div

        function getConstructor() {
            if (TYPE === 'commenting') {
                return window.FastCommentsUI;
            }
            if (TYPE === 'live') {
                return window.FastCommentsLiveChat;
            }
            if (TYPE === 'collab') {
                return window.FastCommentsCollabChat;
            }
        }

        function getScript() {
            if (TYPE === 'commenting') {
                return 'https://cdn.fastcomments.com/js/embed-v2.min.js';
            }
            if (TYPE === 'live') {
                return 'https://cdn.fastcomments.com/js/embed-live-chat.min.js';
            }
            if (TYPE === 'collab') {
                return 'https://cdn.fastcomments.com/js/embed-collab-chat.min.js';
            }
        }

        // Function to ensure script is loaded
        function ensureScriptLoaded() {
            return new Promise((resolve) => {
                // Check if script tag already exists
                let scriptTag = document.getElementById(SCRIPT_ID);

                if (!scriptTag) {
                    console.log('FastComments: Script tag not found, adding dynamically...');
                    scriptTag = document.createElement('script');
                    scriptTag.id = SCRIPT_ID;
                    scriptTag.src = getScript();
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
                } else if (getConstructor()) {
                    // Script tag exists and is already loaded
                    console.log('FastComments: Script already loaded');
                    resolve();
                } else {
                    // Script tag exists but not ready yet
                    console.log('FastComments: Waiting for script to initialize...');
                    scriptTag.addEventListener('load', () => {
                        resolve();
                    });

                    // Fallback in case the script is already loading
                    const checkInterval = setInterval(() => {
                        if (getConstructor()) {
                            clearInterval(checkInterval);
                            resolve();
                        }
                    }, 100);

                    // Timeout after 10 seconds
                    setTimeout(() => {
                        clearInterval(checkInterval);
                        console.warn('FastComments: Script load timeout');
                        resolve();
                    }, 10000);
                }
            });
        }

        // History API modifications for SPA support
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

        let lastInstance;
        let currentUrlId;

        // Main render function
        async function render() {
            let rendered = false;

            // Ensure script is loaded before proceeding
            await ensureScriptLoaded();

            function tryNext() {
                if (rendered) {
                    return;
                }

                // Check if we should render on this page
                if (!VALID_PATTERNS.some(function (pattern) {
                    return window.location.pathname.includes(pattern);
                })) {
                    console.log('FastComments: Not set to load on this page. Waiting.');
                    setTimeout(tryNext, 1000);
                    return;
                }

                // Double-check if available
                if (!getConstructor()) {
                    console.log('FastComments: not ready, waiting...');
                    setTimeout(tryNext, 300);
                    return;
                }

                function getContainer() {
                    let container;
                    if (TARGET_ELEMENT_ID) {
                        container = document.getElementById(TARGET_ELEMENT_ID);
                    } else {
                        // Try to find container with multiple selectors
                        container = document.querySelector('#post-body');
                        if (!container) {
                            container = document.querySelector('#content-container #content-container #post-description');
                        }
                        if (!container) {
                            container = document.querySelector('#post-description');
                        }
                        if (!container) {
                            container = document.querySelector('#content-container');
                        }
                        if (!container) {
                            container = document.querySelector('.post-description'); // mobile
                        }
                    }
                    return container;
                }

                // Look for the target element with specific ID
                let container = getContainer();

                if (container) {
                    console.log('FastComments: Target element found, initializing...');

                    // Get urlId attribute
                    const urlIdAttr = container.getAttribute('urlId');

                    // Check if we need to re-render (urlId changed or first render)
                    if (currentUrlId !== urlIdAttr || !lastInstance) {
                        currentUrlId = urlIdAttr;

                        // Destroy previous instance if exists
                        if (lastInstance) {
                            lastInstance.destroy();
                            // Clear the container content
                            container.innerHTML = '';
                        }

                        // Prepare config
                        const config = {
                            tenantId,
                            showLiveRightAway: true
                        };

                        // Only add urlId to config if it's not "auto"
                        if (urlIdAttr && urlIdAttr !== 'auto') {
                            config.urlId = urlIdAttr;
                            console.log('FastComments: Using urlId:', urlIdAttr);
                        } else {
                            console.log('FastComments: Using auto URL determination');
                        }

                        // Initialize FastComments
                        lastInstance = getConstructor()(container, config);
                        rendered = true;
                    } else {
                        console.log('FastComments: Already rendered with same urlId');
                        rendered = true;
                    }

                    // Monitor if container gets removed or urlId changes
                    const interval = setInterval(function () {
                        const currentContainer = getContainer();
                        if (!currentContainer) {
                            console.log('FastComments: Container removed, will retry...');
                            rendered = false;
                            currentUrlId = null;
                            tryNext();
                            clearInterval(interval);
                        } else {
                            const newUrlId = currentContainer.getAttribute('urlId');
                            if (newUrlId !== currentUrlId) {
                                console.log('FastComments: urlId changed, re-rendering...');
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

        // Initial render
        render();

        // Re-render on location change
        window.addEventListener('locationchange', function () {
            console.log('FastComments: Location changed, updating...');
            render();
        });
    })();
</script>
[inline-code-end]

You can configure the `VALID_PATTERNS` variable to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `streaming` for streaming chat or `collab` for collab chat).
