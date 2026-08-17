[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Quando um usuário comenta com FastComments pela primeira vez, tentaremos buscar seu avatar em <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

No entanto, se não encontrarmos um avatar, ou se o usuário nunca definir um em sua conta, exibiremos uma imagem de avatar padrão estática.

Para especificar sua própria imagem de avatar estática, você pode usar a configuração *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Substituir o Avatar Padrão'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Avatar Padrão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Seção Avatar Padrão da página de personalização do widget, onde você define a URL da imagem de avatar de fallback'; title='Personalizando o Avatar Padrão' app-screenshot-end]

Observe que definir o avatar para um usuário específico, como com SSO, é abordado em sua própria seção.