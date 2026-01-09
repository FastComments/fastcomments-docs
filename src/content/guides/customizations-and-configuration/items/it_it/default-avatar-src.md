[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Quando un utente commenta con FastComments per la prima volta proveremo a recuperare il suo avatar da <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Tuttavia, se non troviamo un avatar, o l'utente non ne imposta mai uno nel proprio account, mostriamo un'immagine avatar predefinita statica.

Per specificare la tua immagine avatar statica puoi usare l'impostazione *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, consulta la sezione "Avatar predefinito".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Nota che la definizione dell'avatar per un utente particolare, ad esempio con SSO, è trattata in una sezione dedicata.