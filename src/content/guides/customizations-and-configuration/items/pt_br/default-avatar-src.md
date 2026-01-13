[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Quando um usuário comenta no FastComments pela primeira vez, tentaremos buscar o avatar dele em <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

No entanto, se não encontrarmos um avatar, ou se o usuário nunca definir um em sua conta, exibimos uma imagem estática de avatar padrão.

Para especificar sua própria imagem de avatar estática, podemos usar a configuração *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Substituir o Avatar Padrão'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Avatar Padrão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Personalizando o Avatar Padrão' app-screenshot-end]

Observe que definir o avatar para um usuário específico, por exemplo com SSO, é abordado em sua própria seção.

---