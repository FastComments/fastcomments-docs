### Namestitev iz GitHub-a

Namestite neposredno iz oznake izdaj (priporočeno, popolnoma reproducibilno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Označite oznako namesto veje, da so gradnje deterministične. Enaka oblika deluje v `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Vsaka označena [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) ima tudi priložen sestavljen paket (wheel), če raje neposredno namestite binarni artefakt.

### Vsebina knjižnice

Ta knjižnica vsebuje dva modula: ustvarjenega API odjemalca in jedrno Python knjižnico, ki vsebuje ročno napisane pripomočke za olajšanje dela z API-jem, vključno s podporo SSO.

- [API Client Library Documentation](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Documentation, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs Zavarovani API-ji

Za API odjemalca obstajajo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa vsebuje metode, ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave naprave/itd. brez avtentikacije. `ModerationApi` ponuja obsežen nabor živih in hitrih moderacijskih API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.