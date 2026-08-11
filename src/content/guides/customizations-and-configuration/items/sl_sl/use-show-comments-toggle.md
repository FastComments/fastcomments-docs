[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments izrisal vnosno polje za komentar in nit komentarjev hkrati. Za prihranek nekaj navpičnega prostora,
bo tudi skril vsa druga zahtevana polja, dokler se z vtičnikom ne interagira.

Vendar pa je lahko pripomoček za komentarje skrit za gumbom, na primer:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Pripomoček za komentarje je zrušen za gumbom, ki prikazuje število komentarjev, dokler bralec ne klikne nanj'; title='Klikni za prikaz komentarjev' app-screenshot-end]

Gumb uporablja različna prevedena besedila, odvisno od tega, ali so komentarji trenutno prikazani ali ne. Če so komentarji skriti, uporablja `translations.SHOW_COMMENTS_BUTTON_TEXT`. Če so
komentarji prikazani, uporablja `translations.HIDE_COMMENTS_BUTTON_TEXT`. Prevajanja lahko vsebujejo besedilo `[count]`, ki bo
zamenjano z lokaliziranim številom.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

To je zasnovano za nadomestitev nastavitve `hideCommentsUnderCountTextFormat`.

Število se posodablja v živo skupaj z nitjo komentarjev. Gumb se ne prikaže, če ni komentarjev.

To je mogoče omogočiti brez kode tako, da ustvarite pravilo prilagoditve in omogočite "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Potrditveno polje za prikaz komentarjev je označeno v pravilu prilagoditve na strani za prilagajanje pripomočka'; title='Omogoči prikaz komentarjev' app-screenshot-end]

---