[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

By default, the FastComments comment widget will automatically detect dark mode on most sites.

When dark mode is detected, FastComments will switch from black text on white backgrounds to white text on a black background. Images will also change.

On page load, the widget will try to determine how dark the background of the page is behind the comment widget. This means that
the page could have a white background, but if you put the comment widget inside a container with a black background, dark mode should
still automatically be enabled to make the comments readable.

However, the detection mechanism, which relies on determining "luminance", may not enable dark mode when you want to. To forcefully enable it, set the
*hasDarkBackground* flag to true as follows:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; code-example-end]

### Toggling Dark Mode

For sites that allow toggling dark mode after the initial page load, this is a bit more involved.

First, all current versions of the Comment widget library (React, Vue) have examples of toggling dark mode in their respective repositories.

For the VanillaJS widget, we will need to do some more work. First, the FastCommentsUI returns an object with the functions "destroy" and "update".

We can simply call the update function every time we want to update the comment widget configuration, as follows. Here is a complete functioning example of toggling
dark mode with the VanillaJS widget.

[inline-code-attrs-start title = 'Toggling Dark Mode Complete Example'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed.js"></script>
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
