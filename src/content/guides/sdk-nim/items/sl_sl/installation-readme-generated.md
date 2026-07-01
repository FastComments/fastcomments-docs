### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

Ta knjižnica vsebuje ustvarjen API odjemalec in SSO pripomočke, ki olajšajo delo z API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

Za API odjemalca obstajajo trije API moduli, `api_default`, `api_public` in `api_moderation`. `api_default` vsebuje metode, ki zahtevajo vaš API ključ, `api_public` vsebuje klice API, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/ipd. brez avtentikacije. Modul `api_moderation` vsebuje metode za nadzorno ploščo moderatorjev.

Modul `api_moderation` ponuja obsežen nabor API-jev za živo in hitro moderiranje. Vsaka metoda `api_moderation` sprejme parameter `sso` in lahko overi prek SSO ali piškotka seje FastComments.com.