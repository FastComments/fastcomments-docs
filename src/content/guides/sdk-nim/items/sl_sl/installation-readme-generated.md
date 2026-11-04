### Uporaba Nimble

```bash
nimble install fastcomments
```

### Gradnja iz izvorne kode

```bash
nimble build
```

### Vsebina knjižnice

Ta knjižnica vsebuje generiranega odjemalca API in pripomočke SSO, ki olajšajo delo z API-jem.

- [Dokumentacija knjižnice odjemalca API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Javni v primerjavi z zaščitenimi API-ji

Za odjemalca API obstajajo trije moduli API: `api_default`, `api_public` in `api_moderation`. Modul `api_default` vsebuje metode, ki zahtevajo vaš API ključ, modul `api_public` pa vsebuje klice API,
ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/idr. brez overjanja. Modul `api_moderation` vsebuje metode za nadzorno ploščo moderatorja.

Modul `api_moderation` ponuja obsežen nabor živih in hitrih API-jev za moderiranje. Vsaka metoda `api_moderation` sprejme parameter `sso` in se lahko overi prek SSO ali piškotka seje FastComments.com.