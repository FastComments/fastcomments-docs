[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is designed to be customized, and the font our widgets use is no exception.

By default, FastComments uses the `system font stack` to look as good as possible on a wide range of devices.

To define your own fonts, see the [Custom CSS documentation](/guide-customizations-and-configuration.html#custom-css).

There you will find a way to define custom CSS, which will allow you to define your desired fonts.

#### How to Define The Font

To override the font, we recommend you define your CSS using the `.fast-comments, textarea` selectors. For example:

[inline-code-attrs-start title = 'Custom External Font Example'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]
