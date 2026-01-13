[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

By default, FastComments allows users to upload images. When a user clicks that image, FastComments will, by default,
open a new tab to show that image in full. Setting this flag to true disables this behavior:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

If you don't plan to capture the image click yourself (see [onImageClicked](#callbacks)), we recommend this is combined with some styling
to remove the appearance that the image can be clicked.
