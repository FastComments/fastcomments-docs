### Korišćenje Nimble

```bash
nimble install fastcomments
```

### Izgradnja iz izvornog koda

```bash
nimble build
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO pomoćne alate koji olakšavaju rad sa API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje tri API modula, `api_default`, `api_public`, i `api_moderation`. `api_default` sadrži metode koje zahtevaju vaš API ključ, a `api_public` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. Modul `api_moderation` sadrži metode za nadzornu ploču moderatora.

Metode u `api_moderation` obuhvataju listanje, prebrojavanje, pretraživanje i izvoz komentara i njihovih zapisa; moderacione radnje kao što su uklanjanje/obnavljanje komentara, označavanje, podešavanje statusa za reviziju/spam/odobrenje, podešavanje glasova i ponovno otvaranje/zatvaranje niti; zabrane (zabrana korisnika u vezi sa komentarom, poništavanje zabrane, pregledi pre zabrane, status i podešavanja zabrane, i brojevi zabranjenih korisnika); i značke i poverenje (dodeljivanje/uklanjanje značke, listanje manuelnih znački, dobijanje/postavljanje korisnikovog faktora poverenja, i dohvat korisnikovog internog profila). Svaka metoda `api_moderation` prihvata parametar `sso` tako da je poziv autentifikovan kao SSO moderator.