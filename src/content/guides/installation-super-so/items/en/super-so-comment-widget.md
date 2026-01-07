## Adding a Live Comment Widget to Your Super.so Notion Articles

In addition to Collab Chat, you can add a traditional comment widget to the bottom of your Notion articles. This allows readers to leave comments and have discussions about the entire article.

### Installation Steps

Copy the following code and paste it in the `Body` section of your Super.so site settings:

[inline-code-attrs-start title = 'Super.so FastComments Live Comment Widget'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let currentPathname = window.location.pathname;
        let currentWidget = null;

        function load() {
            if (!window.FastCommentsUI) {
                console.log('...no script, trying again...');
                return setTimeout(load, 100);
            }

            const contentArea = document.querySelector('.notion-root');
            if (!contentArea || !contentArea.innerHTML || contentArea.innerHTML.length < 100) {
                console.log('...no content, trying again...');
                return setTimeout(load, 100);
            }

            // Clean up existing instance
            if (contentArea.fastCommentsInstance) {
                contentArea.fastCommentsInstance.destroy();
            }

            // Create new target
            const target = document.createElement('div');
            contentArea.append(target);
            currentWidget = target;

            // Initialize FastComments
            contentArea.fastCommentsInstance = FastCommentsUI(target, {
                tenantId: "demo",
                urlId: window.location.pathname
            });

            // Update current pathname
            currentPathname = window.location.pathname;
        }

        // Initial load
        load();

        // Check every 500ms for changes
        setInterval(() => {
            // Reload if pathname changed
            if (window.location.pathname !== currentPathname) {
                console.log('Pathname changed, reloading...');
                load();
                return;
            }

            // Reload if widget was removed
            if (currentWidget && !currentWidget.parentNode) {
                console.log('Widget removed, reloading...');
                load();
                return;
            }

            // Reload if container was emptied
            const contentArea = document.querySelector('.notion-root');
            if (contentArea && contentArea.innerHTML.length < 100) {
                console.log('Container emptied, reloading...');
                load();
            }
        }, 500);
    })();
</script>
[inline-code-end]

### Important Notes

- The comment widget will appear at the bottom of your Notion articles
- Each page gets its own unique comment thread based on the URL path
- Make sure to replace `"demo"` with your actual tenant ID from your FastComments account
- The widget automatically handles Super.so's dynamic page loading
