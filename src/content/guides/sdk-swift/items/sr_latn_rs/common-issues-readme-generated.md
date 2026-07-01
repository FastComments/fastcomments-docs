### 401 greške neautorizovanog pristupa

Ako dobijate 401 greške prilikom korišćenja autentifikovanog API‑ja:

1. **Proverite vaš API ključ**: Uverite se da koristite tačan API ključ sa vašeg FastComments kontrolnog panela
2. **Verifikujte tenant ID**: Proverite da li se tenant ID podudara sa vašim nalogom
3. **Format API ključa**: API ključ treba da bude postavljen kao `x-api-key` zaglavlje u deljenoj konfiguraciji:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Korišćenje pogrešnog API‑ja**: Uverite se da koristite `DefaultAPI` (ne `PublicAPI`) za autentifikovane pozive

### Problemi sa SSO tokenom

Ako SSO tokeni ne rade:

1. **Koristite sigurni režim za produkciju**: Uvek koristite `FastCommentsSSO.createSecure()` sa vašim API ključem za produkciju
2. **Samo na serveru**: Generišite sigurni SSO token na vašem serveru, nikada ne izlažite vaš API ključ klijentima
3. **Proverite podatke o korisniku**: Uverite se da su sva potrebna polja (id, email, username) prosleđena
4. **Isticanje tokena**: Sigurni SSO tokeni sadrže vremensku oznaku i mogu da isteknu. Generišite nove tokena po potrebi.

### SSL/TLS greške

Ako naiđete na SSL/TLS greške:

1. Osigurajte da vaš `Info.plist` dozvoljava HTTPS veze ka fastcomments.com
2. Proverite da li ne koristite izuzetke iz App Transport Security koji bi mogli da blokiraju vezu