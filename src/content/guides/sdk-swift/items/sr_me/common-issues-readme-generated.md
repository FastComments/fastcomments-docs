### 401 Greške neovlaštenog pristupa

Ako dobijate 401 greške prilikom korištenja autentifikovanog API‑ja:

1. **Provjerite svoj API ključ**: Osigurajte da koristite ispravan API ključ sa vaše FastComments kontrolne ploče  
2. **Provjerite tenant ID**: Provjerite da se tenant ID podudara s vašim računom  
3. **Format API ključa**: API ključ treba biti postavljen kao zaglavlje `x-api-key` u zajedničkoj konfiguraciji:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Korištenje pogrešnog API‑ja**: Provjerite da koristite `DefaultAPI` (a ne `PublicAPI`) za autentifikovane pozive  

### Problemi s SSO tokenom

Ako SSO tokeni ne rade:

1. **Koristite sigurni režim za produkciju**: Uvijek koristite `FastCommentsSSO.createSecure()` s vašim API ključem za produkciju  
2. **Samo na serveru**: Generišite sigurni SSO token na vašem serveru, nikada ne izlažite vaš API ključ klijentima  
3. **Provjerite korisničke podatke**: Osigurajte da su pružena sva obavezna polja (id, email, korisničko ime)  
4. **Isticanje tokena**: Sigurni SSO tokeni sadrže vremensku oznaku i mogu isteći. Generišite nove tokene po potrebi.  

### SSL/TLS greške

Ako naiđete na SSL/TLS greške:

1. Osigurajte da vaš Info.plist dozvoljava HTTPS veze prema fastcomments.com  
2. Provjerite da ne koristite izuzetke App Transport Security koji bi mogli blokirati vezu