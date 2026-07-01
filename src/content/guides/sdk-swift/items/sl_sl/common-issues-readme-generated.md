### 401 napake nepooblaščene

Če dobivate napake 401 pri uporabi avtenticiranega API-ja:

1. **Preverite svoj API ključ**: Prepričajte se, da uporabljate pravilen API ključ iz vašega nadzornega plošča FastComments
2. **Preverite ID najemnika**: Prepričajte se, da se ID najemnika ujema z vašim računom
3. **Oblika API ključa**: API ključ naj bo nastavljen kot glava `x-api-key` v skupni konfiguraciji:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **Uporaba napačnega API-ja**: Prepričajte se, da uporabljate `DefaultAPI` (ne `PublicAPI`) za avtenticirane klice

### Težave s SSO žetoni

Če SSO žetoni ne delujejo:

1. **Uporabite varen način za produkcijo**: Vedno uporabljajte `FastCommentsSSO.createSecure()` s svojim API ključem za produkcijo
2. **Samo strežniška stran**: Generirajte varne SSO žetone na vašem strežniku, nikoli ne razkrivajte svojega API ključa odjemalcem
3. **Preverite podatke uporabnika**: Prepričajte se, da so zagotovljena vsa obvezna polja (id, e-pošta, uporabniško ime)
4. **Iztek žetona**: Varni SSO žetoni vsebujejo časovni žig in lahko potečejo. Po potrebi ustvarite sveže žetone.

### SSL/TLS napake

Če naletite na SSL/TLS napake:

1. Prepričajte se, da vaša datoteka Info.plist aplikacije dovoljuje HTTPS povezave do fastcomments.com
2. Preverite, da ne uporabljate izjem App Transport Security, ki bi lahko blokirale povezavo