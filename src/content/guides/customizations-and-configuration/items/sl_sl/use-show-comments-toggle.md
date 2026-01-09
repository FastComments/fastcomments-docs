[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments prikazal polje za vnos komentarja in nit komentarjev hkrati. Da prihrani nekaj navpičnega prostora, bo skril tudi vsa druga obvezna polja, dokler z widgetom ne bo interakcije.

Vendar je mogoče widget za komentarje skriti za gumbom, na primer:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Gumb uporablja različne prevedene nize glede na to, ali so komentarji trenutno prikazani ali ne. Če so komentarji skriti, uporablja `translations.SHOW_COMMENTS_BUTTON_TEXT`. Če so komentarji prikazani, uporablja `translations.HIDE_COMMENTS_BUTTON_TEXT`. Prevodi lahko vsebujejo besedilo `[count]`, ki bo zamenjano z lokaliziranim številom.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

To je zasnovano za zamenjavo konfiguracije `hideCommentsUnderCountTextFormat`.

Štetje se posodablja v živo skupaj z nitjo komentarjev. Gumb se ne prikaže, če komentarjev ni.

To lahko omogočite brez kode z ustvarjanjem pravilnika za prilagoditev in omogočitvijo "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]