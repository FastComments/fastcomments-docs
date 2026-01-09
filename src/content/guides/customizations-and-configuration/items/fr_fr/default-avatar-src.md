[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Lorsqu'un utilisateur commente sur FastComments pour la première fois, nous tenterons de récupérer son avatar depuis <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Cependant, si nous ne trouvons pas d'avatar, ou si l'utilisateur n'en définit jamais dans son compte, nous affichons une image d'avatar par défaut statique.

Pour spécifier votre propre image d'avatar statique, nous pouvons utiliser le réglage *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, consultez la section « Avatar par défaut ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Notez que la définition de l'avatar pour un utilisateur particulier, par exemple via SSO, est traitée dans une section dédiée.

---