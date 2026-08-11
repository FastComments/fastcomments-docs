---
Privzeto FastComments ne dovoljuje iframe-ov v komentarjih. Ko omogočite vdelave medijev, lahko komentatorji prilepijo kodo vdelave (odsek `<iframe>`) od zaupanja vrednih ponudnikov, kot so YouTube, Vimeo, SoundCloud in Spotify, in ta se bo prikazala v vrstici v komentarju.

Zaradi varnosti to ni nastavitvena zastavica gradnika na strani odjemalca. Gre za nastavitev na strežniku, ki se preveri, ko se vsak komentar shrani, zato je ni mogoče vklopiti s strani. Dovoljeni so le iframe-ji, ki kažejo na vgrajen seznam zaupanja vrednih ponudnikov. Vsak drug iframe je odstranjen.

To je izvedeno brez kode, na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Nastavitev vdelav medijev je vklopljena na strani za prilagajanje gradnika, kar omogoča komentatorjem lepljenje zaupanja vrednih iframe vdelav'; title='Dovoli vdelave medijev' app-screenshot-end]

### Dodajanje lastnih ponudnikov

Če želite dovoliti vdelave od ponudnika, ki ni na vgrajenem seznamu zaupanja vrednih, dodajte njegovo ime gostitelja v polje "Dodatna domena vdelav" na isti strani. Ta imena gostiteljev so dovoljena poleg vgrajenih ponudnikov. Ujemanje je natančno, zato vključite celotno ime gostitelja (na primer, player.example.com). Vse, kar ne navedete, ostane blokirano.

Tako običajno polje za komentar kot urejevalnik WYSIWYG podpirata lepljenje vdelave. V urejevalniku WYSIWYG je vdelava vstavljena kot odstranljiv blok.
---