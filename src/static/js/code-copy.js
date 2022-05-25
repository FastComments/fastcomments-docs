(function () {
    document.addEventListener('click', async function (e) {
        if (e.target && e.target.classList.contains('copy')) {
            const response = await fetch(`/snippets/${e.target.attributes['data-snippet-id'].value}`);
            const text = await response.text();
            await navigator.clipboard.writeText(text);
            e.target.querySelector('span').innerText = 'Copied!';
            setTimeout(function () {
                e.target.querySelector('span').innerText = 'Copy';
            }, 5000);
        }
    });
})();
