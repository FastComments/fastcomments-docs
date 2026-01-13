[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Avec FastComments, tout le texte du widget de commentaires est personnalisable.

Vous pouvez remplacer une seule portion de texte, comme le bouton d'envoi, ou tout le texte du widget de commentaires.

Par défaut, le texte du widget de commentaires est traduit en fonction de la locale de l'utilisateur. Toutefois, nous pouvons remplacer le texte si nous sommes convaincus
que notre base d'utilisateurs utilise la même locale/langue, par exemple :

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Toutes les traductions personnalisables se trouvent <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">ici</a> sous l'onglet "options avancées".

Cependant, il existe une façon plus simple, via l'interface de personnalisation du widget. Là, nous pouvons simplement trouver le texte qui s'affiche dans le widget de commentaires pour la locale EN_US, et spécifier
un remplacement.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Toutes les substitutions de traductions affectent actuellement toutes les locales.