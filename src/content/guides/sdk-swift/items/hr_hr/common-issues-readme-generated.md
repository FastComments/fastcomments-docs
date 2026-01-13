### 401 Neovlaštene pogreške

Ako dobivate 401 pogreške pri korištenju autentificiranog API-ja:

1. **Provjerite svoj API key**: Uvjerite se da koristite ispravan API key na svojoj FastComments nadzornoj ploči
2. **Provjerite tenant ID**: Provjerite da se tenant ID podudara s vašim računom
3. **API key format**: API key treba biti postavljen na API clientu:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Korištenje pogrešnog API-ja**: Provjerite koristite li `DefaultAPI` (ne `PublicAPI`) za autentificirane pozive

### Problemi s SSO tokenima

Ako SSO tokeni ne rade:

1. **Koristite siguran način za produkciju**: Uvijek koristite `FastCommentsSSO.createSecure()` s vašim API keyem za produkciju
2. **Samo na serveru**: Generirajte sigurne SSO tokene na svom serveru, nikada ne izlažite svoj API key klijentima
3. **Provjerite podatke o korisniku**: Provjerite da su navedena sva obavezna polja (id, email, username)
4. **Istek tokena**: Sigurni SSO tokeni sadrže vremensku oznaku i mogu isteći. Generirajte nove tokene po potrebi.

### SSL/TLS pogreške

Ako naiđete na SSL/TLS pogreške:

1. Provjerite omogućava li Info.plist vaše aplikacije HTTPS veze prema fastcomments.com
2. Provjerite da ne koristite izuzetke App Transport Security koji bi mogli blokirati vezu