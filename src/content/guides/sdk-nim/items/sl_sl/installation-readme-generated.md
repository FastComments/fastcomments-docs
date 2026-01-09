### Uporaba Nimble

```bash
nimble install fastcomments
```

### Gradnja iz izvorne kode

```bash
nimble build
```

### Vsebina knjižnice

Ta knjižnica vsebuje generiran API odjemalec in SSO orodja, ki poenostavijo delo z API-jem.

- [Dokumentacija knjižnice odjemalca API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni in zaščiteni API-ji

Za API odjemalca so na voljo dva modula API: `api_default` in `api_public`. `api_default` vsebuje metode, ki zahtevajo vaš API ključ, in `api_public` vsebuje klice API-ja
ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/itd brez overjanja.