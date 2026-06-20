FastComments Rust SDK je sestavljen iz več modulov:

- **Modul odjemalca** - API-odjemalec za FastComments REST API-je
  - Celotne definicije tipov za vse modele API-ja
  - Trije API-odjemalci, ki pokrivajo vse metode FastComments:
    - `default_api` (**DefaultApi**) - Metode, avtenticirane z API ključem, za strežniško uporabo
    - `public_api` (**PublicApi**) - javni, brez API-ključa metode, ki jih je varno klicati iz brskalnikov in mobilnih aplikacij
    - `moderation_api` (**ModerationApi**) - metode, ki podpirajo nadzorno ploščo moderatorja, vključno z moderacijo komentarjev (seznam, štetje, iskanje, dnevniki, izvoz), ukrepi moderacije (odstrani/obnovi, označi, nastavi status pregleda/spama/odobritve, glasovi, ponovno odpri/zaključi nit), prepovedmi (prepoved iz komentarja, razveljavitev, povzetki pred prepovedjo, status/preferenc prepovedi, število prepovedanih uporabnikov) ter značkami in zaupanje (podeli/odstrani značke, ročne značke, pridobi/nastavi faktor zaupanja, notranji uporabniški profil). Vsaka moderacijska metoda sprejme parameter `sso`, tako da je klic mogoče izvesti v imenu moderatorja, avtenticiranega preko SSO.
  - Popolna podpora async/await z tokio
  - Za podrobno dokumentacijo API-ja glejte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md)

- **Modul SSO** - Strežniški pripomočki za Single Sign-On
  - Varno generiranje žetonov za avtentikacijo uporabnikov
  - Podpora za preproste in varne SSO načine
  - Podpisovanje žetonov, ki temelji na HMAC-SHA256

- **Osnovni tipi** - Skupne definicije tipov in pripomočki
  - Modeli komentarjev in strukture metapodatkov
  - Konfiguracije uporabnikov in najemnikov
  - Pomožne funkcije za pogoste operacije