### Korištenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generiran API klijent i SSO pomoćne alate koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni vs Zaštićeni API-ji

Za API klijenta postoje tri API modula, `api_default`, `api_public`, i `api_moderation`. `api_default` sadrži metode koje zahtijevaju vaš API ključ, a `api_public` sadrži API pozive koje je moguće izvršiti izravno iz preglednika/mobilnog uređaja/itd. bez autentikacije. Modul `api_moderation` sadrži metode za nadzornu ploču moderatora.

Metode `api_moderation` obuhvaćaju listanje, brojanje, pretraživanje i izvoz komentara i njihovih zapisa; moderacijske radnje poput uklanjanja/obnavljanja komentara, označavanja, postavljanja statusa za pregled/spam/odobrenje, podešavanja glasova i ponovno otvaranje/zaključavanje razgovora; zabrane (zabrana korisnika od komentiranja, povratak zabrane, sažeci prije zabrane, status i postavke zabrane te brojevi zabranjenih korisnika); te značke i povjerenje (dodjeljivanje/uklanjanje značke, listanje ručnih znački, dobivanje/postavljanje faktora povjerenja korisnika i dohvaćanje internog profila korisnika). Svaka `api_moderation` metoda prihvaća `sso` parametar tako da je poziv ovjeren kao SSO moderator.