Obstajata dva načina, kako preprečiti uporabnikom komentiranje na vaši strani z uporabo FastComments.

Prvi način je, da če že poznate njihov e-poštni naslov, ga lahko vnesete na stran <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">Prepovedani uporabniki</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Do te strani je mogoče dostopati prek Moderiraj komentarje -> Prepovedani uporabniki

Ko bomo uporabnika prepovedali, lahko izberemo tip, bodisi Trajna ali Trajna prikrita prepoved:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Drugi način za prepoved uporabnika je s klikom gumba za prepoved, ki se nahaja pri vsakem komentarju na strani Moderacija komentarjev.

Ko kliknete gumb za prepoved, se vam prikažejo nekatere možnosti, kjer lahko določimo tip prepovedi in trajanje.

### E-poštni aliasi

Pri prepovedi uporabnika po e-poštnem naslovu FastComments samodejno prezre `+` alias-e. Na primer, prepoved `user+alias@gmail.com` bo
prav tako prepovedala `user@gmail.com` in katerokoli drugo `+` različico tega naslova, na primer `user+other@gmail.com`.

### Prikrite prepovedi

Prikrita prepoved je vrsta prepovedi, ki daje videz, da je bil uporabnikov komentar ali glas uspešno shranjen, ko v resnici ni bil. To je lahko
zaželeno v določenih situacijah.

### Prepoved prek IP naslova

Če se najemnik noče izključiti, FastComments podpira prepoved po IP z shranjevanjem zgoščene (hashed) različice IP naslova komentatorja.