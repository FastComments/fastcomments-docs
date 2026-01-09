[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Par défaut, FastComments affichera le widget de commentaires dans la locale déterminée par le système et le navigateur de l'utilisateur.

Lorsqu'un utilisateur commente ou se connecte, nous mettons à jour sa dernière locale utilisée et l'utilisons également pour l'envoi des courriels.

Cela affecte la façon dont le widget de commentaires est traduit pour l'utilisateur. La locale est composée de la langue et de la région de l'utilisateur, donc configurer la locale change généralement la langue utilisée pour afficher le texte à l'utilisateur.

#### Via l'interface utilisateur

Cela peut être défini à l'aide de l'interface de personnalisation du widget. Voir l'option "Locale / Language" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via le code

Cela peut être remplacé par la locale souhaitée.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Langues prises en charge et codes de locales

[Vous pouvez trouver la liste complète des langues prises en charge et les codes de locales correspondants ici.](/guide-supported-languages.html#supported-languages)

### Remarque SSO

Si vous utilisez SSO, vous pourriez vouloir transmettre la locale de l'utilisateur dans le user object, afin que les courriels et autres éléments soient correctement localisés pour cet utilisateur.

---