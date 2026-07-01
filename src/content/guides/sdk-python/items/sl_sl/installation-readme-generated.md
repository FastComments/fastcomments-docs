### Namestitev s GitHub-a

Namestite neposredno iz oznake izdaj (priporočeno, popolnoma reproducibilno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Zakažite oznako namesto veje, da so gradnje določljive. Enak zapis deluje v `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Vsaka označena [GitHub izdaja](https://github.com/fastcomments/fastcomments-python/releases) ima tudi priloženo zgrajeno wheel, če raje neposredno namestite binarni artefakt.

### Vsebina knjižnice

Ta knjižnica vsebuje dva modula: ustvarjenega API odjemalca in jedrno Python knjižnico, ki vsebuje ročno napisane pripomočke, ki olajšajo delo z API-jem, vključno s podporo za SSO.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s SSO primeri](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni in zaščiteni API-ji

Za API odjemalca so na voljo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` vsebuje metode, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave itd. brez avtentikacije. `ModerationApi` ponuja obsežen nabor živo in hitro moderiranje API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali FastComments.com seje piškotka.