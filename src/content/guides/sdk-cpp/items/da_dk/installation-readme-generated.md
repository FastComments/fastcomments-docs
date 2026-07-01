### Installer afhængigheder

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Byg fra kildekode

```bash
mkdir build
cd build
cmake ..
make
```

### Installation

```bash
sudo make install
```

### Biblioteksindhold

Dette bibliotek indeholder den genererede API-klient og SSO‑værktøjerne, som gør arbejdet med API'en lettere.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Offentlige vs sikrede API'er

For API‑klienten er der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API‑nøgle, og `PublicApi` indeholder  
metoder, der kan kaldes direkte fra en browser/mobil enhed/osv uden godkendelse. `ModerationApi` leverer en omfattende suite af live og hurtige moderations‑API'er. Hver `ModerationApi`‑metode accepterer en `sso`‑parameter og kan godkendes via SSO eller en FastComments.com sessions‑cookie.