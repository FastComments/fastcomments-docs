[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

For sites that allow toggling dark mode after the initial page load, this is a bit more involved.

First, all current versions of the Comment widget library (React, Vue) have examples of toggling dark mode in their respective repositories.

For the VanillaJS widget, we will need to do some more work. First, the FastCommentsUI returns an object with the functions "destroy" and "update".

We can simply call the update function every time we want to update the comment widget configuration, as follows. Here is a complete functioning example of toggling
dark mode with the VanillaJS widget.

[inline-code-attrs-start title = 'Komplet eksempel på at skifte mørk tilstand'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---