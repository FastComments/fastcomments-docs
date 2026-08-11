Podrazumevano FastComments ne dozvoljava iframe‑ove u komentarima. Kada omogućite medijske embedove, komentatori mogu da nalepi kod za embed (the `<iframe>` snippet) od pouzdanih provajdera kao što su YouTube, Vimeo, SoundCloud i Spotify, i on će se prikazati inline u komentaru.

Iz sigurnosnih razloga, ovo nije konfiguraciona zastavica widgeta na klijentskoj strani. To je podešavanje na serverskoj strani, koje se validira prilikom čuvanja svakog komentara, pa se ne može uključiti sa stranice. Dozvoljeni su samo iframe‑ovi koji ukazuju na ugrađenu listu pouzdanih provajdera. Svaki drugi iframe se uklanja.

Ovo se radi bez koda, na stranici za prilagođavanje widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Podešavanje medijskih embedova uključeno na stranici za prilagođavanje widgeta, omogućavajući komentatorima da nalepaju pouzdane iframe embedove'; title='Dozvoli medijske embedove' app-screenshot-end]

### Dodavanje sopstvenih provajdera

Ako želite da dozvolite embedove od provajdera koji nije na ugrađenoj listi pouzdanih, dodajte njegov hostname u polje „Additional Embed Domains“ na istoj stranici. Ovi hostnames su dozvoljeni uz ugrađene provajdere. Poklapanje je tačno, pa uključite kompletan hostname (na primer, player.example.com). Sve što ne navedete ostaje blokirano.

I obična kutija za komentar i WYSIWYG editor podržavaju lepljenje embedova. U WYSIWYG editoru embed se ubacuje kao blok koji se može ukloniti.