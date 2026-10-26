[related-parameter-start name = 'commentTranslation'; type = "'off' | 'auto' | 'manual'"; related-parameter-end]

By default, FastComments translates the widget's own interface (buttons, labels, error messages) into the viewer's language, but comments are shown exactly as they were written.

Comment Translation adds machine translation of the comments themselves, so a community that writes in more than one language can read every comment in its own.

There are two modes:

- **Automatic**: comments written in a language other than the widget's language are shown translated. A small "Translated from Spanish" line appears under the comment with a **Show original** link that switches between the translation and the untranslated text.
- **Manual**: every comment gets a **Translate** link. Clicking it translates that one comment into the viewer's language, with the same **Show original** toggle afterwards. Comments that are already in the viewer's language say so instead.

The target language is the widget's language: the **Locale / Language** setting on the same customization rule if it is set, otherwise the visitor's browser language. Nothing is required from the visitor, no account, extension or extra software.

This is configured without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Comment Translation dropdown on the widget customization page with Off, Automatic and Manual options'; title='Comment Translation' app-screenshot-end]

It can also be set in code:

[code-example-start config = {commentTranslation: 'auto'}; linesToHighlight = [6]; title = 'Enabling Automatic Comment Translation'; code-example-end]

Translations are produced by an AI model, run through the same sanitization and link rules as the original comment, and cached on the comment. A comment is translated at most once per target language; if the author edits it, the cached translation is discarded and the comment is translated again on the next view. Comments that are too long, or whose language cannot be detected, are shown untranslated.

Usage is billed as LLM tokens, like other AI features, and appears in the same place in your billing details.
