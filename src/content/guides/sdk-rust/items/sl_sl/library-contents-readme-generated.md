FastComments Rust SDK sestavlja več modulov:

- **Client Module** – API odjemalec za FastComments REST API‑je
  - Celovite definicije tipov za vse modele API‑ja
  - Trije API odjemalci, ki pokrivajo vse FastComments metode:
    - `default_api` (**DefaultApi**) – metode, avtenticirane s ključem API, za uporabo na strežniku
    - `public_api` (**PublicApi**) – javne metode brez ključa API, ki so varne za klic iz brskalnikov in mobilnih aplikacij
    - `moderation_api` (**ModerationApi**) – obsežna zbirka živo in hitro delujočih moderacijskih API‑jev. Vsaka metoda Moderation sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.
  - Polna podpora za async/await s tokio
  - Oglejte si [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) za podrobno dokumentacijo API‑ja

- **SSO Module** – strežniška orodja za enotno prijavo (Single Sign-On)
  - Varen generiranje žetonov za avtentikacijo uporabnikov
  - Podpora tako preprostem kot varnemu načinu SSO
  - Podpisovanje žetonov na osnovi HMAC‑SHA256

- **Core Types** – skupne definicije tipov in orodja
  - Modeli komentarjev in strukture metapodatkov
  - Konfiguracije uporabnikov in najemnikov
  - Pomožne funkcije za pogoste operacije