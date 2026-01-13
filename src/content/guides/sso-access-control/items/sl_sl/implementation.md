#### Označevanje uporabnikov v drugih skupinah

Če dva uporabnika pripadata dvema različnima množicama skupin in ni preseka, se ne bosta mogla `@mention`-ati.

Če uporabnik ročno vpiše `@mention` in odda svoj komentar, bo ostalo navadno besedilo. Drugi uporabnik ne bo označen.

#### Vzdrževanje skupin

`Groups` so definirane z uporabo API virov `Pages` in `SSOUsers`.

`Pages` API lahko pokličete, da definirate nabor skupin, ki smejo dostopati do strani. Privzeto imajo dostop vse skupine in uporabniki, ki ne pripadajo nobeni skupini.

Podobno lahko `SSOUsers` API pokličete, da definirate skupine, povezane z vsakim uporabnikom.

Za oba vira ni omejitev, kdaj je mogoče nastaviti ali posodobiti skupine.

Če je cilj le omejiti, da se uporabniki med seboj ne `@mention`-ajo, potem `Pages` ni potrebno upoštevati.

##### Opomba!

Določanje in posodabljanje SSO uporabniških skupin ne zahteva uporabe API-ja in se lahko namesto tega samodejno posodobi z določanjem group ids v SSO payload, poslanem v pripomoček za komentarje. Vendar za velike sezname skupin to ni priporočljivo, saj bi moral uporabnik poslati ta payload ob vsakem nalaganju strani.