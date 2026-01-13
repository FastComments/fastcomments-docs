[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

By default, FastComments allows users to upload images. When a user clicks that image, FastComments will, by default,
open a new tab to display that image at full size. Setting this flag to true disables this behavior:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

If you don't plan to handle the image click yourself (see [onImageClicked](#callbacks)), we recommend combining this with styling
to remove the appearance that the image is clickable.