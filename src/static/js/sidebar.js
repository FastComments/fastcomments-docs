(function () {
    const headings = document.querySelectorAll('.index-guide-item h2');
    let lastVisibleHeadingText = '';
    window.addEventListener('scroll', function () {
        const pageTop = window.scrollY;
        const pageBottom = pageTop + window.innerHeight;
        for (const heading of headings) {
            const elementTop = heading.offsetTop;
            const elementBottom = elementTop + heading.getBoundingClientRect().height;

            if ((pageTop < elementTop) && (pageBottom > elementBottom)) {
                if (lastVisibleHeadingText === heading.textContent) {
                    return;
                }
                for (const item of document.querySelectorAll('.sidebar-item')) {
                    if (item.querySelector('a').textContent.trim() === heading.textContent.trim()) {
                        item.classList.add('selected');
                        document.querySelector('.sidebar').scrollTop = item.offsetTop - 200;
                    } else {
                        item.classList.remove('selected');
                    }
                }
                lastVisibleHeadingText = heading.textContent;
                break;
            }
        }
    });
})();
