(function () {
    document.addEventListener('click', async function (e) {
        if (e.target && e.target.classList.contains('copy')) {
            const textLines = Array.from(e.target.parentNode.parentNode.querySelectorAll('.line .line-content'))
                .map(function(lineElement) {
                    return lineElement.innerText;
                });

            let text = '';
            for (let i = 0; i < textLines.length; i++) {
                const line = textLines[i];
                if (line.endsWith('\n')) {
                    text += line;
                } else {
                    text += line;
                    const isLast = i === textLines.length - 1;
                    if (!isLast) {
                        text += '\n';
                    }
                }
            }

            await navigator.clipboard.writeText(text);
            e.target.querySelector('span').innerText = 'Copied!';
            setTimeout(function () {
                e.target.querySelector('span').innerText = 'Copy';
            }, 5000);
        }
    });
})();
