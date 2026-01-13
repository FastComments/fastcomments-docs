FastComments é uma plataforma localizada. Todos os nossos widgets, e-mails e notificações são localizados.

Localizado significa que mostramos um idioma e uma formatação diferentes, com base na localização do usuário
e no idioma preferido. Determinamos isso com base nas informações que o navegador do usuário nos fornece.

Podemos personalizar o texto no e-mail acessando a aba `Translations`, selecionando um `Locale`
e editando o texto. O texto que é alterado em relação ao padrão é destacado na UI. Você pode
alternar entre locales e salvar ao final, sem perder as alterações.

Texto localizado é acessado através do objeto `TEXT`, por exemplo: `<%= TEXT.INTRO %>`.

### Nota sobre SSO

Para integrações SSO, se `locale` não for especificado, ele será atualizado sempre que o usuário
acessar o widget de comentários com um locale diferente. Isso significa que a preferência de idioma
deles é atualizada automaticamente, e futuros e-mails serão enviados nesse locale.

Isso também pode ser definido manualmente fornecendo `locale` no payload SSO.