By default, FastComments translates the widget's own interface (buttons, labels, error messages) into the viewer's language, but comments are shown exactly as they were written.

Comment Translation adds translation of the comments themselves, so a community that writes in more than one language can read every comment in its own.

There are two modes:

- **Automatski**: comments written in a language other than the widget's language are shown translated. A "Prevedeno sa španskog" line appears under the comment with a **Prikaži original** link that switches between the translation and the untranslated text.
- **Ručni**: every comment gets a **Prevedi** link. Clicking it translates that one comment into the viewer's language, with the same **Prikaži original** toggle afterwards. Comments that are already in the viewer's language say so instead.

The target language is the widget's language: the [Locale / Language](#locale) setting on the same customization rule if it is set, otherwise the visitor's browser language. Nothing is required from the visitor, no account, extension or extra software, and it works the same in the embedded widget and on standalone comment pages.

This is turned on per <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagođavanja</a>, so it can apply to your whole site, one domain, or a single page. It is off by default.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Padajući meni Prevođenje komentara na stranici za prilagođavanje widgeta sa opcijama Isključeno, Automatski i Ručni'; title='Prevođenje komentara' app-screenshot-end]

Translation is a server-side setting. Passing `commentTranslation` in the widget's JavaScript configuration does not enable it; the rule does.

Each comment is translated at most once per target language and the result is stored with the comment, so a comment read by a thousand Spanish speakers is translated once. If the author or a moderator edits the comment, the stored translation is discarded and the comment is translated again on the next view. Translated text goes through the same sanitization and link rules as the original comment. Comments that are too long, or whose language cannot be detected, are shown untranslated.

Usage is billed as LLM tokens at the same rate as [AI Agents](/guide-ai-agents.html#plans-and-eligibility) and appears in the same place in your billing details.