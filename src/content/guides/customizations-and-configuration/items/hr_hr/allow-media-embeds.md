---
Prema zadanim postavkama FastComments ne dopušta iframeove u komentarima. Kada omogućite medijska embedovanja, komentatori mogu zalijepiti kod za embed ( `<iframe>` isječak) od pouzdanih pružatelja poput YouTubea, Vimeo‑a, SoundClouda i Spotifyja, i on će se prikazati unutar komentara.

Iz sigurnosnih razloga, ovo nije konfiguracijska zastavica widgeta na klijentskoj strani. To je postavka na strani poslužitelja, provjerena prilikom spremanja svakog komentara, pa se ne može uključiti s stranice. Dopušteni su samo iframeovi koji upućuju na ugrađeni popis pouzdanih pružatelja. Svi ostali iframeovi se uklanjaju.

Ovo se radi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Postavka medijskih ugrađivanja uključena na stranici prilagodbe widgeta, omogućuje komentatorima lijepljenje pouzdanih iframe ugrađivanja'; title='Dozvoli medijske embedove' app-screenshot-end]

### Dodavanje vlastitih pružatelja

Ako želite dopustiti embedove od pružatelja koji nije na ugrađenom popisu pouzdanih, dodajte njegov naziv hosta u polje "Dodatne domene za embed" na istoj stranici. Ovi nazivi hosta su dopušteni uz ugrađene pružatelje. Podudaranje je točno, pa uključite puni naziv hosta (na primjer, player.example.com). Sve što ne navedete ostaje blokirano.

I obični okvir za komentar i WYSIWYG uređivač podržavaju lijepljenje embedova. U WYSIWYG uređivaču embed se umeće kao uklonjivi blok.