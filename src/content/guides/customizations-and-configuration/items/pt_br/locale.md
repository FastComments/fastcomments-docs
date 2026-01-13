[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Por padrão, o FastComments renderizará o widget de comentários no locale determinado pelo sistema e navegador do usuário.

Quando um usuário comenta ou faz login, atualizamos seu último locale utilizado e também o usamos para o envio de e-mails.

Isso impacta como o widget de comentários é traduzido para o usuário. Locale consiste na língua e região do usuário, portanto configurar o locale irá
normalmente alterar o idioma usado para mostrar texto ao usuário.

#### Via a interface do usuário

Isto pode ser definido usando a UI de personalização do widget. Veja a opção "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Código

Isto pode ser sobrescrito com um locale desejado.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Idiomas suportados e códigos de locale

[Você pode encontrar a lista completa de idiomas suportados e os códigos de locale correspondentes aqui.](/guide-supported-languages.html#supported-languages)

### Observação sobre SSO

Se você estiver usando SSO, talvez queira passar o locale do usuário no user object, para que e-mails e outras coisas sejam localizados corretamente para eles.

---