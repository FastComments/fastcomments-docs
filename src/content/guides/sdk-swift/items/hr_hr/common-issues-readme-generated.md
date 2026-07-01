### 401 Greške neovlaštenog pristupa

Ako primate 401 greške pri korištenju autentificiranog API‑ja:

1. **Provjerite svoj API ključ**: Provjerite koristite li ispravan API ključ iz vašeg FastComments nadzorne ploče
2. **Provjerite ID najmodavca**: Provjerite podudara li se ID najmodavca s vašim računom
3. **Format API ključa**: API ključ treba postaviti u zaglavlje `x-api-key` u zajedničkoj konfiguraciji:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Korištenje pogrešnog API‑ja**: Provjerite koristite li `DefaultAPI` (ne `PublicAPI`) za autentificirane pozive

### Problemi s SSO tokenom

Ako SSO tokeni ne rade:

1. **Koristite siguran način u proizvodnji**: Uvijek koristite `FastCommentsSSO.createSecure()` s vašim API ključem u proizvodnji
2. **Samo na poslužitelju**: Generirajte sigurne SSO tokene na vašem poslužitelju, nikada ne izlažite vaš API ključ klijentima
3. **Provjerite podatke o korisniku**: Provjerite da su svi potrebni podaci (id, e‑mail, korisničko ime) pruženi
4. **Istjecanje tokena**: Sigurni SSO tokeni uključuju vremensku oznaku i mogu isteći. Generirajte nove tokene po potrebi.

### SSL/TLS greške

Ako se susretnete s SSL/TLS greškama:

1. Osigurajte da vaš `Info.plist` aplikacije dopušta HTTPS veze na fastcomments.com
2. Provjerite da ne koristite iznimke App Transport Security koje bi mogle blokirati vezu