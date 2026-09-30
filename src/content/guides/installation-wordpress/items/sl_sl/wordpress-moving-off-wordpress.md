---
Če selite svoje spletno mesto iz WordPressa in želite FastComments na novem mestu, WordPress vtičnika ne potrebujete. Izvozite svoje komentarje iz WordPressa, nato naložite datoteko na [Stran za uvoz](https://fastcomments.com/auth/my-account/manage-data/import) v nadzorni plošči FastComments.

Podpiramo dva formata izvoza iz WordPressa.

### WordPress XML (Priporočeno)

To je datoteka iz vgrajenega izvoznika WordPressa, zato dodatni vtičnik ni potreben.

1. V vašem WordPress administracijskem vmesniku pojdite na `Tools -> Export`.
2. Izberite `All content` in kliknite `Download Export File`.
3. Na FastComments [Stran za uvoz](https://fastcomments.com/auth/my-account/manage-data/import) izberite `WordPress (.xml)` in naložite datoteko.

Vsak komentar je povezan z URL-jem objave, na kateri je bil napisan, kar je že v datoteki.

Uvoz ohrani ime avtorja, e‑pošto in spletno stran, datum, vsebino, nit odgovora ter ali je bil komentar odobren. Avatarji komentatorjev so
preneseni iz Gravatara. Glasovi niso del tega formata.

### WordPress CSV

To je datoteka iz [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Namestite vtičnik v vašem WordPress administracijskem vmesniku in izvozite svoje komentarje kot CSV.
2. Zamenjajte vsako vrednost `comment_post_ID` z URL-jem objave.
3. Na FastComments [Stran za uvoz](https://fastcomments.com/auth/my-account/manage-data/import) izberite `WordPress (.csv)` in naložite datoteko.

Vsak komentar je povezan s stolpcem `comment_post_ID`. WordPress napolni ta stolpec z ID-jem objave, vaš novi spletni naslov pa nima WordPress ID-jev objav,
zato korak 2 zamenja to z URL-jem.

Uvoz ohrani ime avtorja, e‑pošto in spletno stran, datum, vsebino, nit odgovora ter ali je bil komentar odobren. Avatarji komentatorjev so
preneseni iz Gravatara. Prav tako ohrani WordPressov spam zastavico ter všečke in nesprejemanja wpDiscuz, ko so ti vključeni v datoteko.

### Ujemanje komentarjev z vašimi novimi stranmi

Če vaš novi spletni naslov ohranja iste URL-je kot vaš WordPress spletni naslov, se komentarji prikažejo na ustreznih straneh brez dodatne nastavitve.

Če se domena spremeni, po uvozu zaženite [Orodje za selitev domene](/guide-migrations.html#migrating-domains). Če se posamezni URL-ji strani spremenijo, lahko
[selite vsako stran](/guide-migrations.html#migrating-pages) z njegovega starega URL-ja na novega.

Za množične selitve strani, kot je odstranjevanje domene iz vrednosti, ki jo posredujete polju [urlId](/guide-customizations-and-configuration.html#url-id) gradnika komentarja, [odprite zahtevek za podporo](https://fastcomments.com/auth/my-account/help) in mi bomo to uredili za vas.

### Preden preklopite

Uvoz lahko izvedete kolikokrat želite. Ponovni uvoz iste datoteke [ne ustvari podvojenih](/guide-migrations.html#importing-data), zato lahko
uvozite enkrat za testiranje novega spletnega mesta, nato pa ponovno uvozite z najnovejšimi komentarji tik pred preklopom.

Za izvozne datoteke, večje od 1 GB, [stopite v stik s podporo](https://fastcomments.com/auth/my-account/help).

Za dodajanje FastComments na vaše novo spletno mesto, si oglejte [Namestitveni vodnik](/guide-installation.html).

---