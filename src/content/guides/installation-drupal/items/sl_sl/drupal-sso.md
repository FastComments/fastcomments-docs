FastComments se poveže z Drupalovim sistemom uporabnikov preko SSO oziroma enotne prijave. Vaši uporabniki se prijavijo na vaše Drupal spletno mesto, modul pa samodejno posreduje njihovo identiteto v FastComments. Ni treba ustvarjati dodatnih računov, ni začetne sinhronizacije.

Modul podpira tri načine SSO, nastavljene pod `Administration > Configuration > Content > FastComments`.

### Brez

Brez SSO. Uporabniki komentirajo kot gostje ali ustvarijo FastComments račun. Uporabite to, če je vaše mesto javno in ne potrebujete povezovanja komentarjev z Drupal uporabniki.

### Enostaven

Posreduje Drupalovo uporabniško ime, e-pošto in avatar v FastComments brez strežniške verifikacije. Ni potreben API Secret. Primerno za notranja ali nizko-rizična mesta.

### Varen (priporočeno)

Uporablja [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) za preverjanje vsake uporabniške identitete z FastComments. To je način, ki ga želite, ko imate nastavljen API Secret, in je edini način, ki prepreči obiskovalcu, da se lažno predstavlja za drugega uporabnika.

Uporabniška identiteta se posreduje v FastComments vsakič, ko uporabnik ogleda nit komentarjev. Ni začetne ali stalne sinhronizacije, ki bi jo bilo treba izvajati.

<sup>(Optional)</sup> Dodajte vaše skrbnike v [Uporabniki & Skrbniki](https://fastcomments.com/auth/my-account/users) in moderatorje v [Moderatorji komentarjev](https://fastcomments.com/auth/my-account/moderate-comments/moderators), da izboljšate njihovo izkušnjo in omogočite sledenje statistik moderatorjem.

Za podrobnejši vpogled, kako SSO deluje, glejte [SSO oddelek](/guide-customizations-and-configuration.html#sso) dokumentacije o prilagoditvah.