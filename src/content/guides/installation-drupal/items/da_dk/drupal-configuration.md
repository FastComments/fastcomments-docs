All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Krævet

- **Tenant ID** - Din FastComments Tenant ID. Find den under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Påkrævet for Secure SSO, webhook‑verifikation og side‑synkronisering. Find den under [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## Gem API Secret

Modulet læser API Secret fra den første af disse kilder, der har en værdi:

1. **Key module** - Installer [Key](https://www.drupal.org/project/key), opret en *Authentication* nøgle, og vælg den under **API Secret key** på indstillingsformularen. Nøglen kan understøttes af en miljøvariabel eller en fil uden for webroden.
2. **settings.php** - Tilføj `$config['fastcomments.settings']['api_secret'] = 'your-secret';` til `settings.php` eller `settings.local.php`. Hemmeligheden indgår aldrig i eksporteret konfiguration.
3. **The settings form** - Indtast den i **API Secret**‑feltet. Dette gemmer værdien i `fastcomments.settings`, så den ender i konfigurationseksport. Drupal‑statusrapporten advarer, når dette er den aktive kilde.

Formularen viser, hvilken kilde der er aktiv. Hvis feltet efterlades tomt, bevares den gemte hemmelighed. Marker **Fjern den gemte API Secret i konfigurationen** for at slette den efter at have flyttet hemmeligheden til Key‑modulet eller `settings.php`.

## Kommentarstil

Vælg den widget, der passer til, hvordan du vil have folk til at tale på dit site.

- **Live Comments** - Kommentarer i realtid med trådning.
- **Streaming Chat** - Live chat‑grænseflade, god til begivenheder og livestreams.
- **Collab Chat** - Tekstmarkering‑annotation i hovedindholdsområdet. Besøgende markerer tekst og starter en diskussion i kontekst.
- **Collab Chat + Comments** - Både collab chat og standard kommentarer på samme side.

## SSO‑tilstand

- **None** - Ingen SSO. Brugere kommenterer som gæster eller opretter en FastComments‑konto.
- **Simple** - Overfører Drupal‑brugerinfo (navn, e‑mail, avatar) til FastComments uden server‑side verifikation.
- **Secure** - Bruger HMAC‑SHA256 til at verificere Drupal‑brugere med FastComments. Anbefales når du har en API Secret konfigureret.

Se afsnittet `Single Sign-On (SSO)` for detaljer.

## Andre indstillinger

- **CDN URL** - Standard er `https://cdn.fastcomments.com`.
- **Site URL** - Standard er `https://fastcomments.com`.
- **Email notifications** - Send en e‑mail til en indholdsforfatter, når en ny kommentar er postet på deres indhold.

For EU‑databeskyttelse, se afsnittet `EU Data Residency`.