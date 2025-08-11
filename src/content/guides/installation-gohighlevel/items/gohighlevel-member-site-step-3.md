Now we're going to copy our FastComments code. Try using the "Copy" button on the right, as the code is quite large to work properly
with GoHighLevel:

[inline-code-attrs-start title = 'GoHighLevel FastComments Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        const VALID_PATTERNS = ['/post'];

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

        function render() {
            let rendered = false;

            function tryNext() {
                if (rendered) {
                    return;
                }
                if (!VALID_PATTERNS.some(function(pattern) {
                    return window.location.pathname.includes(pattern);
                })) {
                    console.log('FastComments: Not set to load on this page. Waiting.');
                    setTimeout(tryNext, 1000);
                    return;
                }
                if (!window.FastCommentsUI) {
                    console.log('FastComments: script not ready, waiting...');
                    setTimeout(tryNext, 300);
                    return;
                }
                let container = document.querySelector('#post-body');
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
                if (container) {
                    console.log('FastComments: container found, updating...');
                    if (document.querySelector('.fastcomments-wrapper')) {
                        document.querySelector('.fastcomments-wrapper').remove();
                    }
                    const target = document.createElement('div');
                    target.classList.add('fastcomments-wrapper', 'col-span-8');
                    container.append(target);
                    FastCommentsUI(target, {
                        tenantId: "demo",
                        showLiveRightAway: true
                    });
                    rendered = true;
                    const interval = setInterval(function() {
                        const doesContainerStillExist = document.querySelector('.fastcomments-wrapper');
                        if (!doesContainerStillExist) {
                            rendered = false;
                            tryNext();
                            clearInterval(interval);
                        }
                    }, 1000);
                } else {
                    console.log('FastComments: container not found, waiting...');
                    setTimeout(tryNext, 300);
                }
            }

            tryNext();
        }

        render();

        window.addEventListener('locationchange', function () {
            console.log('Updating FastComments.');
            render();
        });
    })();
</script>
[inline-code-end]

You can configure the `VALID_PATTERNS` variable to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.
