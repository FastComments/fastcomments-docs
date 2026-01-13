---
### 401 Neavtorizirane napake

Če prejemate 401 napake pri uporabi avtenticiranega API-ja:

1. **Preverite svoj API ključ**: Prepričajte se, da uporabljate pravilen API ključ iz svojega FastComments nadzornega panela
2. **Preverite tenant ID**: Prepričajte se, da se tenant ID ujema z vašim računom
3. **Oblika API ključa**: API ključ mora biti nastavljen na API client:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **Uporaba napačnega API-ja**: Prepričajte se, da uporabljate `DefaultAPI` (ne `PublicAPI`) za avtenticirane klice

### Težave s SSO žetoni

Če SSO žetoni ne delujejo:

1. **Uporabite varen način za produkcijo**: Vedno uporabite `FastCommentsSSO.createSecure()` z vašim API ključem za produkcijo
2. **Samo na strežniku**: Ustvarjajte varne SSO žetone na vašem strežniku, nikoli ne izpostavljajte svojega API ključa klientom
3. **Preverite podatke uporabnika**: Poskrbite, da so zagotovljena vsa zahtevana polja (id, email, username)
4. **Potek veljavnosti žetona**: Varnim SSO žetonom je priložen časovni žig in lahko potečejo. Po potrebi ustvarite nove žetone.

### SSL/TLS napake

Če naletite na SSL/TLS napake:

1. Prepričajte se, da Info.plist vaše aplikacije dovoljuje povezave HTTPS do fastcomments.com
2. Preverite, da ne uporabljate izjem App Transport Security, ki bi lahko blokirale povezavo
---