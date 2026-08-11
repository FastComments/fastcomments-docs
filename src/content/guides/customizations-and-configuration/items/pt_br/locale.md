[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Por padrão, o FastComments renderizará o widget de comentários no locale determinado pelo sistema e navegador do usuário.

Quando um usuário comenta ou faz login, atualizamos o locale usado pela última vez por ele e também o utilizamos para enviar e‑mails.

Isso afeta como o widget de comentários é traduzido para o usuário. O locale consiste no idioma e região do usuário, portanto, configurar o locale geralmente mudará o idioma usado para exibir o texto ao usuário.

#### Via a UI

Isso pode ser definido usando a UI de personalização do widget. Veja a opção "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Menu suspenso Locale / Language na página de personalização do widget usado para substituir o locale detectado do visitante'; title='Alterando o Locale / Language' app-screenshot-end]

#### Via Código

Isso pode ser sobrescrito com um locale desejado.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Definindo Manualmente o Locale do Usuário'; code-example-end]

### Idiomas Suportados e Códigos de Locale

[Você pode encontrar a lista completa de idiomas suportados e os códigos de locale correspondentes aqui.](/guide-supported-languages.html#supported-languages)

### Observação sobre SSO

Se você estiver usando SSO, talvez queira passar o locale do usuário no objeto do usuário, para que e‑mails e outras coisas sejam localizados corretamente para ele.

---