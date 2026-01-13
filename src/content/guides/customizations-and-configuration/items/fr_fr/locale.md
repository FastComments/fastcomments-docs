[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Par défaut, FastComments affichera le widget de commentaires dans la locale déterminée par le système et le navigateur de l'utilisateur.

Lorsqu'un utilisateur commente ou se connecte, nous mettons à jour sa dernière locale utilisée et l'utilisons également pour l'envoi des e-mails.

Cela affecte la manière dont le widget de commentaires est traduit pour l'utilisateur. La locale se compose de la langue et de la région de l'utilisateur ; configurer la locale modifiera généralement la langue utilisée pour afficher le texte à l'utilisateur.

#### Via l'interface utilisateur

Cela peut être défini à l'aide de l'interface de personnalisation du widget. Voir l'option "Locale / Language" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via le code

Cela peut être remplacé par la locale souhaitée.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Langues et codes de locale pris en charge

[Vous pouvez trouver la liste complète des langues prises en charge et des codes de locale correspondants ici.](/guide-supported-languages.html#supported-languages)

### Remarque SSO

Si vous utilisez SSO, vous souhaiterez peut-être passer la locale de l'utilisateur dans l'objet utilisateur, afin que les e-mails et autres éléments soient correctement localisés pour cet utilisateur.