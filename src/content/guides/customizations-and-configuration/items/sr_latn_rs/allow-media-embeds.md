Podrazumevano, FastComments ne dozvoljava iframe elemente u komentarima. Kada omogućite ugradnje medija, komentatori mogu nalepiti embed kod (`<iframe>` snippet) od pouzdanih provajdera kao što su YouTube, Vimeo, SoundCloud i Spotify, i on će se prikazati direktno u komentaru.

Iz bezbednosnih razloga, ovo nije konfiguraciona zastavica widgeta na strani klijenta. To je podešavanje na serverskoj strani, koje se validira pri čuvanju svakog komentara, tako da se ne može uključiti sa same stranice. Dozvoljeni su samo iframe elementi koji upućuju na ugrađenu listu pouzdanih provajdera. Svaki drugi iframe se uklanja.

Ovo se radi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Dodavanje sopstvenih provajdera

Ako želite dozvoliti embed-ove sa provajdera koji nije na ugrađenoj listi pouzdanih, dodajte njegov hostname u polje "Additional Embed Domains" na istoj stranici. Ovi hostnames se dozvoljavaju pored ugrađenih provajdera. Uparivanje je tačno, zato uključite puni hostname (na primer, player.example.com). Sve što ne navedete ostaje blokirano.

I obično polje za komentare i WYSIWYG editor podržavaju lepljenje embeda. U WYSIWYG editoru embed se umeće kao blok koji se može ukloniti.