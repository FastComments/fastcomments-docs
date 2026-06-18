Privzeto FastComments ne dovoljuje iframe-ov v komentarjih. Ko omogočite vdelave medijev, lahko komentatorji prilepijo vdelavno kodo (the `<iframe>` snippet) od zaupanja vrednih ponudnikov, kot so YouTube, Vimeo, SoundCloud in Spotify, in se bo prikazala v vrstici v komentarju.

Zaradi varnosti to ni konfiguracijska zastavica widgeta na strani odjemalca. Gre za nastavitev na strežniku, ki se preveri ob shranjevanju vsakega komentarja, zato je ni mogoče vklopiti iz strani. Dovoljeni so le iframe-i, ki kažejo na vgrajen seznam zaupanja vrednih ponudnikov. Vsak drug iframe se odstrani.

To se naredi brez kode, na strani za prilagajanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Dodajanje lastnih ponudnikov

Če želite dovoliti vdelave od ponudnika, ki ni na vgrajenem seznamu zaupanja vrednih ponudnikov, dodajte njegovo ime gostitelja v polje "Additional Embed Domains" na isti strani. Ta imena gostiteljev so dovoljena poleg vgrajenih ponudnikov. Ujemanje je natančno, zato vključite polno ime gostitelja (na primer player.example.com). Vse, česar ne navedete, ostane blokirano.

Tako navadno polje za komentar kot WYSIWYG urejevalnik podpirata prilepitev vdelave. V WYSIWYG urejevalniku se vdelava vstavi kot odstranljiv blok.

---