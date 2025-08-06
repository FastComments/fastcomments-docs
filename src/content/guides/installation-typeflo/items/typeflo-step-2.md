In the **Footer** section of the Custom Code tab, paste the following code:

[inline-code-attrs-start title = 'Typeflo.io Live Commenting Code Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        console.log('Starting FastComments');

        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                console.log('fastcomments found');
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                console.log('subscribeSection', subscribeSection);
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    console.log('fullWidthSection', fullWidthSection);
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Paste Code in Footer Section</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Paste FastComments Code in Footer Section" />
</div>

After pasting the code, click the **Save** button to apply your changes.

Note: This code includes logic to dynamically place the FastComments widget in the optimal location on your Typeflo.io blog posts. Other code snippets will not work properly with Typeflo.io's layout.

Remember to replace `'demo'` with your actual FastComments tenant ID after signing up. If you're logged in to FastComments.com, it should already be replaced.