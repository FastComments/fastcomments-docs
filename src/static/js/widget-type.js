(function() {
    const widgetId = new URLSearchParams(window.location.search).get('wId');
    if (widgetId) {
        const idToConstructorMap = {
            'comment-ui': 'FastCommentsUI',
            'comment-ui-v2': 'FastCommentsUI',
            'live-chat': 'FastCommentsLiveChat',
        };
        const idToJSMap = {
            'comment-ui': '/js/embed.min.js',
            'comment-ui-v2': '/js/embed-v2.min.js',
            'live-chat': '/js/embed-live-chat.min.js',
        };
        const constructor = idToConstructorMap[widgetId];
        const jsFile = idToJSMap[widgetId];
        if (constructor && jsFile) {
            document.querySelectorAll('.code .line').forEach(function(lineElement) {
                if (lineElement.innerHTML.includes('FastCommentsUI')) {
                    lineElement.innerHTML = lineElement.innerHTML.replace('FastCommentsUI', constructor);
                } else if (lineElement.innerHTML.includes('/js/embed-v2.min.js')) {
                    lineElement.innerHTML = lineElement.innerHTML.replace('/js/embed-v2.min.js', jsFile);
                }
            });
        }
    }
})();
