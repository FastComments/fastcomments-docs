With FastComments SSO Access Control, sometimes referred to as RBAC, uporabnike lahko omejimo, da dostopajo le do določenih strani ali nitk komentarjev. Poleg tega se lahko uporabniki med seboj `@mention`ajo le v isti skupini.

## Podrobneje

V skupine lahko uvrstimo `Users` in po želji tudi `Pages`.

Ko so `Users` uvrščeni v skupine in je v nastavitvah pripomočka omogočeno `Limit Comments by SSO User Groups`, bodo uporabniki videli le komentarje uporabnikov iz svojih skupin in bodo lahko `@mention`ali le uporabnike v istih skupinah.

Poleg tega je mogoče `Pages` uvrstiti v skupine, in takrat lahko uporabniki dostopajo le do komentarjev za strani, do katerih imajo dostop.

Temu pravimo skupine na ravni uporabnika oziroma skupine na ravni strani.

Torej, katera je prava za vas?

#### Uporabite skupine na ravni uporabnika, če...

- Želite uporabiti isto vrednost `urlId` (URL strani ali ID članka), vendar omejiti komentarje glede na skupino.
- Na primer, želite imeti skupini "Nov uporabnik" in "Izkušen uporabnik", ki si nikoli ne bi videle komentarjev drug drugega na istih straneh.

#### Uporabite skupine na ravni strani, če...

- Vaše skupine imajo določene strani.
- Na primer, uporabniki v skupini "Javne strani" nikoli ne bi smeli pregledovati člankov, označenih kot "Top Secret".