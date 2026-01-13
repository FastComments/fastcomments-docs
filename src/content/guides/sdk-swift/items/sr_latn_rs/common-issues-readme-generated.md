### 401 Neovlašćene greške

Ako dobijate 401 greške kada koristite autentifikovani API:

1. **Proverite vaš API ključ**: Uverite se da koristite tačan API ključ iz vašeg FastComments kontrolnog panela
2. **Proverite tenant ID**: Uverite se da se tenant ID poklapa sa vašim nalogom
3. **Format API ključa**: API ključ treba biti postavljen na API klijentu:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Korišćenje pogrešnog API-ja**: Uverite se da koristite `DefaultAPI` (ne `PublicAPI`) za autentifikovane pozive

### Problemi sa SSO tokenima

Ako SSO tokeni ne rade:

1. **Koristite sigurni režim za produkciju**: Uvek koristite `FastCommentsSSO.createSecure()` sa vašim API ključem za produkciju
2. **Samo na serverskoj strani**: Generišite sigurne SSO tokene na vašem serveru, nikada ne izlažite vaš API ključ klijentima
3. **Proverite podatke o korisniku**: Uverite se da su svi obavezni podaci (id, email, username) obezbeđeni
4. **Isticanje tokena**: Sigurni SSO tokeni uključuju vremensku oznaku i mogu isteći. Generišite sveže tokene po potrebi.

### SSL/TLS greške

Ako naiđete na SSL/TLS greške:

1. Proverite da Info.plist vaše aplikacije dozvoljava HTTPS konekcije ka fastcomments.com
2. Proverite da ne koristite izuzetke App Transport Security koji bi mogli blokirati konekciju