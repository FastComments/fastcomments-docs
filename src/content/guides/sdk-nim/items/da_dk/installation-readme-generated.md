### Brug af Nimble

```bash
nimble install fastcomments
```

### Byg fra kildekoden

```bash
nimble build
```

### Bibliotekets indhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjer, som gør det nemmere at arbejde med API'et.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Offentlige vs Sikrede API'er

For API-klienten er der to API-moduler, `api_default` og `api_public`. `api_default` indeholder metoder, der kræver din API-nøgle, og `api_public` indeholder API-opkald
der kan foretages direkte fra en browser/mobil enhed/etc. uden autentificering.