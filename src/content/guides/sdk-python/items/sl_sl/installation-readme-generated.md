### Namestitev iz GitHub

Namestite neposredno iz oznake izdaj (priporočeno, popolnoma reproducibilno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Označite oznako namesto veje, da so gradnje deterministične. Enaka oblika deluje v `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Vsaka označena [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) ima tudi priložen sestavljen paket, če raje neposredno namestite binarni artefakt.

### Vsebina knjižnice

Ta knjižnica vsebuje dva modula: generiranega API odjemalca in jedrno Python knjižnico, ki vsebuje ročno napisane pripomočke za olajšanje dela z API-jem, vključno s podporo SSO.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs varovani API-ji

Za API odjemalca so na voljo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` vsebuje metode, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/itd. brez avtentikacije. `ModerationApi` ponuja obsežen nabor živo in hitro delujočih moderacijskih API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.