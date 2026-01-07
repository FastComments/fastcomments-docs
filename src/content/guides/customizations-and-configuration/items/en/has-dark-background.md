[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

By default, the FastComments comment widget will automatically detect dark mode on most sites.

When dark mode is detected, FastComments will switch from black text on white backgrounds to white text on a black background. Images will also change.

On page load, the widget will try to determine how dark the background of the page is behind the comment widget. This means that
the page could have a white background, but if you put the comment widget inside a container with a black background, dark mode should
still automatically be enabled to make the comments readable.

However, the detection mechanism, which relies on determining "luminance", may not enable dark-mode when you want to. To forcefully enable it, set the
*hasDarkBackground* flag to true as follows:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]
