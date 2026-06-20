### Installer afhængigheder

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Byg fra kildekoden

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

### Bibliotekets indhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjer, der gør arbejdet med API'et lettere.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Offentlige vs. sikrede API'er

For API-klienten er der tre klasser, `DefaultApi`, `PublicApi`, og `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` indeholder
metoder, der kan kaldes direkte fra en browser/mobil enhed osv. uden godkendelse. The `ModerationApi` contains methods that power the moderator dashboard - listing,
counting, searching, exporting and pulling logs for comments, moderation actions (fjern/gendan, marker, sæt review-/spam-/godkendelsesstatus, juster stemmer, genåbn/luk tråde),
bans (udeluk fra en kommentar, fortryd udelukkelser, forudgående udelukkelsesoversigter, udelukkelsesstatus og præferencer, antal udelukkede brugere), og badges & trust (tildel/fjern badges, manuelle badges, få/indstil tillidsfaktor,
brugerens interne profil). Every `ModerationApi` method accepts an `sso` parameter so the call is performed on behalf of an SSO-authenticated moderator.