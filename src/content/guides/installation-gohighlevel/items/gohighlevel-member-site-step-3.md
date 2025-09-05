Now we're going to copy our FastComments code. Try using the "Copy" button on the right, as the code is quite large to work properly
with GoHighLevel. Note that by default this will only show on page URLs that contain `/post`. You can add more pages in the `VALID_PATTERNS` line, or set it to `[]` to enable on all pages.

[inline-code-attrs-start title = 'GoHighLevel FastComments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script>
    (function () {
        const tenantId = 'demo';
        const VALID_PATTERNS = ['/post']; // only show on these pages. This is ignored if you set TARGET_ELEMENT_ID.
        const TYPE = 'commenting'; // set to one of: commenting, live, collab. You can also configure this per-page.
        const TARGET_ELEMENT_ID = ''; // you can set this to add the widget to a specific area of the page by adding a separate HTML element with a div
        const SCRIPT_ID = 'fastcomments-embed'; // don't change this

        function getType() {
            if (TARGET_ELEMENT_ID) {
                const container = document.getElementById(TARGET_ELEMENT_ID);
                if (container) {
                    return container.getAttribute('type');
                } else {
                    console.log('FastComments: Type not available yet.');
                    return null;
                }
            }
            return TYPE;
        }

        function getConstructor() {
            const type = getType();
            if (!type) {
                return null;
            }
            if (type === 'commenting') {
                return window.FastCommentsUI;
            }
            if (type === 'live') {
                return window.FastCommentsLiveChat;
            }
            if (type === 'collab') {
                return window.FastCommentsCollabChat;
            }
        }

        function getScript() {
            const type = getType();
            if (type === 'commenting') {
                return 'https://cdn.fastcomments.com/js/embed-v2.min.js';
            }
            if (type === 'live') {
                return 'https://cdn.fastcomments.com/js/embed-live-chat.min.js';
            }
            if (type === 'collab') {
                return 'https://cdn.fastcomments.com/js/embed-collab-chat.min.js';
            }
        }

        function getContainer() {
            let container;
            if (TARGET_ELEMENT_ID) {
                container = document.getElementById(TARGET_ELEMENT_ID);
                if (container && container.getAttribute('type') === 'collab') {
                    const hasContent = container.textContent && container.textContent.trim().length > 0;
                    if (hasContent) {
                        return container;
                    } else {
                        container = getContentContainer();
                    }
                }
            } else {
                container = getContentContainer();
            }
            return container;
        }

        // Function to ensure script is loaded
        function ensureScriptLoaded() {
            return new Promise(async (resolve) => {
                // Check if script tag already exists
                let scriptTag = document.getElementById(SCRIPT_ID);

                if (!scriptTag) {
                    await new Promise((resolve) => {
                        const interval = setInterval(() => {
                            if (!getType()) {
                                return; // wait
                            }
                            // wait for this so we can accurately determine what script/constructor to fetch
                            const container = getContentContainer();
                            if (container) {
                                resolve();
                                clearInterval(interval);
                            }
                        }, 100);
                    });
                    console.log('FastComments: Script tag not found, adding dynamically...');
                    scriptTag = document.createElement('script');
                    scriptTag.id = SCRIPT_ID;
                    scriptTag.src = getScript();
                    scriptTag.async = true;

                    scriptTag.onload = () => {
                        console.log('FastComments: Script loaded successfully', scriptTag.src);
                        resolve();
                    };

                    scriptTag.onerror = () => {
                        console.error('FastComments: Failed to load script', scriptTag.src);
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
                        if (getConstructor(getContainer())) {
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

        function getContentContainer() {
            let container = null;
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
            return container;
        }

        let lastInstance;
        let currentUrlId;

        // Main render function
        async function render() {
            let rendered = false;

            // Ensure script is loaded before proceeding
            await ensureScriptLoaded();

            // Look for the target element with specific ID
            let container = getContainer();

            function tryNext() {
                if (rendered) {
                    return;
                }

                // Check if we should render on this page
                if (!TARGET_ELEMENT_ID && !VALID_PATTERNS.some(function (pattern) {
                    return window.location.pathname.includes(pattern);
                })) {
                    console.log('FastComments: Not set to load on this page. Waiting.');
                    setTimeout(tryNext, 1000);
                    return;
                }

                container = getContainer(); // important to re-fetch

                if (container) {
                    // Double-check if available
                    if (!getConstructor()) {
                        console.log('FastComments: not ready, waiting...');
                        setTimeout(tryNext, 300);
                        return;
                    }
                    console.log('FastComments: Target element found, initializing...');

                    // Get urlId attribute
                    const urlIdAttr = container.getAttribute('urlid');

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
                            if (getType() === 'collab' && (!getContentContainer() || !getContentContainer().classList.contains('fastcomments-collab-chat-container'))) {
                                console.log('FastComments: collab chat removed!');
                                container = null; // collab chat was removed
                                rendered = false;
                                tryNext();
                                clearInterval(interval);
                            }
                            const newUrlId = currentContainer.getAttribute('urlid');
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

### Different Comment Box Types

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `live` for streaming chat or `collab` for collab chat).

### Putting The Comment Box Where You Want

Let's say you want to put comment boxes on specific parts of the page and not the default locations.
Change this line:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

To:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Then in the GHL editor, click the "code" button and add where you want the comments to go:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Different Comment Box Type Per-Page

Let's say you want users to highlight and discuss pieces of text, or use the streaming chat UI instead.

First follow the steps above in "Putting The Comment Box Where You Want".

Note in that small snippet there's `type="commenting"`.

If you want to enable collab chat for example change type to `type="collab"`.

### Only Show On Specific Pages

If you don't set don't set `TARGET_ELEMENT_ID`, you can instead configure the `VALID_PATTERNS` variable, to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.

### Configuring Collab Chat

You can tell collab chat to only add collaborative functionality around HTML inside a specific area, for example, let's say you
add the footer code above and then add this div in the post/page content to enable collab chat:

[inline-code-attrs-start title = 'Collab Chat With Specified Content'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Then the paragraph element inside the `<div>` will have collab chat enabled, and nothing else on the page. If you don't
put any content in the `<div>` then it will enable collab chat on the entire post body.
