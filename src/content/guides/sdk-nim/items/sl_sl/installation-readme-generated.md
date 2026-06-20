### Uporaba Nimble

```bash
nimble install fastcomments
```

### Gradnja iz izvorne kode

```bash
nimble build
```

### Vsebina knjižnice

Ta knjižnica vsebuje generiran odjemalec API in SSO orodja za lažje delo z API-jem.

- [Dokumentacija knjižnice odjemalca API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni in zavarovani API-ji

Za odjemalca API so na voljo trije moduli API, `api_default`, `api_public` in `api_moderation`. `api_default` vsebuje metode, ki zahtevajo vaš API ključ, `api_public` pa vsebuje klice API-ja, ki jih je mogoče izvajati neposredno iz brskalnika/mobilne naprave/itd. brez overjanja. Modul `api_moderation` vsebuje metode za nadzorno ploščo moderatorja.

Metode `api_moderation` zajemajo izpisovanje, štetje, iskanje in izvoz komentarjev in njihovih dnevnikov; moderacijske ukrepe, kot so odstranjevanje/obnavljanje komentarjev, prijavljanje, nastavljanje statusa pregleda/spama/odobritve, spreminjanje glasov ter ponovno odpiranje/zapiranje nití; prepovedi (blokiranje uporabnika glede komentiranja, razveljavitev prepovedi, povzetki pred prepovedjo, stanje in nastavitve prepovedi ter število prepovedanih uporabnikov); in značke ter zaupanje (podeljevanje/odstranjevanje značke, seznam ročnih značk, pridobivanje/nastavljanje faktorja zaupanja uporabnika in pridobitev notranjega profila uporabnika). Vsaka metoda `api_moderation` sprejme parameter `sso`, tako da je klic avtenticiran kot SSO moderator.