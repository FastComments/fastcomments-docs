(function () {
    const sidebar = document.querySelector('.sidebar');
    document.querySelector('.sidebar-open-button').addEventListener('click', function() {
        sidebar.classList.toggle('open');
    });
    const headings = document.querySelectorAll('.index-guide-item h2');
    let lastVisibleHeadingText = '';
    function updateSidebar () {
        const pageTop = window.scrollY;
        const pageBottom = pageTop + window.innerHeight;
        for (const heading of headings) {
            const elementTop = heading.offsetTop;
            const elementBottom = elementTop + heading.getBoundingClientRect().height;

            // We don't want to check if the heading is entirely in view, just that the bottom of it is. This results
            // in a better experience.
            if ((pageTop < elementBottom) && (pageBottom > elementBottom)) {
                if (lastVisibleHeadingText === heading.textContent) {
                    return;
                }
                for (const item of sidebar.querySelectorAll('.sidebar-item')) {
                    if (item.querySelector('a').textContent.trim() === heading.textContent.trim()) {
                        item.classList.add('selected');
                        sidebar.scrollTop = item.offsetTop - 200;
                    } else {
                        item.classList.remove('selected');
                    }
                }
                lastVisibleHeadingText = heading.textContent;
                break;
            }
        }
    }
    window.addEventListener('scroll', updateSidebar);
    updateSidebar();

    sidebar.addEventListener('click', function(event) {
        if (event.target.tagName.toLowerCase() === 'a') {
            if (sidebar.classList.contains('open')) {
                sidebar.classList.remove('open');
            }
        }
    });
})();
