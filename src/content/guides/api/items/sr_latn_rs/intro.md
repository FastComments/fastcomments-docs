### FastComments API

FastComments pruža API za interakciju sa mnogim resursima. Napravite integracije sa našom platformom, ili čak izgradite sopstvene klijente!

U ovoj dokumentaciji ćete naći sve podržane resurse API-ja dokumentovane sa njihovim tipovima zahteva i odgovora.

Za Enterprise korisnike, sav pristup API-ju se beleži u Audit Log-u.

### Generisani SDK-ovi

FastComments sada generiše [API Spec](https://fastcomments.com/js/swagger.json) iz našeg koda (ovo još nije potpuno završeno, ali uključuje mnoge API-je).

Takođe sada imamo SDK-ove za popularne jezike:

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### Autentifikacija

API se autentifikuje prosleđivanjem vašeg [api key](https://fastcomments.com/auth/my-account/api-secret) kao ili `X-API-KEY` zaglavlje ili `API_KEY` parametar upita. Takođe će vam biti potreban vaš `tenantId` za pozivanje API-ja. Ovo se može preuzeti sa iste stranice kao i vaš api key.

### Bezbednosna napomena

Ove rute su namenjene pozivanju sa **servera**. __NE POZIVAJTE__ ih iz pregledača. Ako to uradite, izložićete vaš API ključ - to će obezbediti potpuni pristup vašem nalogu svima koji mogu videti izvorni kod stranice!

#### Opcija autentifikacije 1 - Zaglavlja

- Zaglavlje: `X-API-KEY`
- Zaglavlje: `X-TENANT-ID`

#### Opcija autentifikacije 2 - Parametri upita

- Parametar upita: `API_KEY`
- Parametar upita: `tenantId`

### Čitanje sopstvenih zapisa

FastComments pruža Active-Active dostupnost. Zahtevi iz vašeg data centera se usmeravaju na [najbližu tačku prisustva](https://sophon.fastcomments.com/) u odnosu na vašu. Ovo je automatski, i obično možete očekivati semantiku čitanja sopstvenih izmena. Ako želite biti sigurni da čitate svoje sopstvene izmene, možete fiksirati vaše zahteve na određeni region koristeći taj region kao njegov API host (međutim ovo obično nije potrebno za većinu integracija):

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

Imajte na umu da, ako ovo uradite, možda ćete želeti da definišete fallback, jer smo u prošlosti zastarevali ulazne čvorove i koristili nova imena za prebacivanje.