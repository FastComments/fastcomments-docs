---
Por padrão, o FastComments não limita os idiomas usados para comentar. 

Pode ser desejável limitar os idiomas que uma comunidade usa.

Isso pode ser configurado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Seletor de idiomas permitidos na página de personalização do widget para limitar quais idiomas os comentários podem usar'; title='Idiomas Permitidos' app-screenshot-end]

O sistema analisará o comentário e determinará seu idioma, e então o comparará com a lista de permitidos.

Se o comentário for escrito em um idioma que não é permitido, uma mensagem de erro localizada será exibida. 

---