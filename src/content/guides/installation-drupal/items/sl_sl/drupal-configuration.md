All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Zahtevano

- **Tenant ID** – Vaš FastComments Tenant ID. Najdete ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** – Potrebno za varno SSO, preverjanje webhooka in sinhronizacijo strani. Najdete ga pod [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Shranjevanje API skrivnosti

Modul prebere API Secret iz prvega od teh virov, ki ima vrednost:

1. **Key module** – Namestite [Key](https://www.drupal.org/project/key), ustvarite ključ *Authentication* in ga izberite pod **API Secret key** v obrazcu nastavitev. Ključ je lahko podprt z okoljsko spremenljivko ali datoteko izven korena spletnega strežnika.
2. **settings.php** – Dodajte `$config['fastcomments.settings']['api_secret'] = 'your-secret';` v `settings.php` ali `settings.local.php`. Skrivnost se nikoli ne vključi v izvoženo konfiguracijo.
3. **The settings form** – Vnesite jo v polje **API Secret**. To shrani vrednost v `fastcomments.settings`, zato se pojavi v izvozu konfiguracije. Drupalovo poročilo o stanju opozori, ko je to aktivni vir.

Obrazec prikazuje, kateri vir je aktiven. Če pustite polje prazno, se ohrani shranjena skrivnost. Označite **Remove the API Secret stored in configuration**, da jo izbrišete po premiku skrivnosti v modul Key ali `settings.php`.

## Slog komentiranja

Izberite gradnik, ki ustreza načinu, kako želite, da ljudje komunicirajo na vašem spletnem mestu.

- **Live Comments** – Komentarji v realnem času.
- **Streaming Chat** – Vmesnik za klepet v živo, primeren za dogodke in prenose v živo.
- **Collab Chat** – Oznaka izbranega besedila v glavnem vsebinskem območju. Obiskovalci označijo besedilo in začnejo razpravo v kontekstu.
- **Collab Chat + Comments** – Tako collab chat kot standardni komentarji na isti strani.

## Način SSO

- **None** – Brez SSO. Uporabniki komentirajo kot gostje ali ustvarijo FastComments račun.
- **Simple** – Posreduje informacije o Drupal uporabniku (ime, e‑mail, avatar) FastComments brez strežniškega preverjanja.
- **Secure** – Uporablja HMAC‑SHA256 za preverjanje Drupal uporabnikov s FastComments. Priporočeno, ko je konfigurirana API Secret.

Za podrobnosti glejte razdelek `Single Sign-On (SSO)`.

## Druge nastavitve

- **CDN URL** – Privzeto `https://cdn.fastcomments.com`.
- **Site URL** – Privzeto `https://fastcomments.com`.
- **Email notifications** – Pošlji e‑mail avtorju vsebine, ko je na njegovo vsebino objavljen nov komentar.

Za rezidenčnost podatkov v EU glejte razdelek `EU Data Residency`.