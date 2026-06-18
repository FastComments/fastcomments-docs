Prema zadanim postavkama FastComments ne dopušta iframe elemente u komentarima. Kada omogućite ugrađivanje medija, komentatori mogu zalijepiti kod za ugradnju (isječak `<iframe>`) od pouzdanih pružatelja poput YouTube, Vimeo, SoundCloud i Spotify, i on će se prikazati izravno u komentaru.

Iz sigurnosnih razloga, ovo nije konfiguracijska opcija widgeta na strani klijenta. To je postavka na strani poslužitelja, provjerava se pri spremanju svakog komentara, pa se ne može uključiti s same stranice. Dozvoljeni su samo iframeovi koji upućuju na unaprijed ugrađeni popis pouzdanih pružatelja. Bilo koji drugi iframe se uklanja.

To se radi bez koda, na stranici za prilagodbu widgeta:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Dodavanje vlastitih pružatelja

Ako želite dopustiti ugradnje s pružatelja koji nije na ugrađenom popisu pouzdanih, dodajte njegov naziv hosta u polje "Dodatne domene za ugradnju" na istoj stranici. Ti nazivi hostova su dopušteni uz ugrađene pružatelje. Usporedba je točna, stoga uključite puni naziv hosta (na primjer, player.example.com). Sve što ne navedete ostaje blokirano.

I običan okvir za komentare i WYSIWYG uređivač podržavaju lijepljenje ugradnje. U WYSIWYG uređivaču ugradnja se umeće kao blok koji se može ukloniti.