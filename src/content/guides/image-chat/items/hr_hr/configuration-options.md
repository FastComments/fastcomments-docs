### Pregled

FastComments Image Chat proširuje standardni FastComments widget za komentare, pa nasljeđuje sve opcije konfiguracije iz osnovnog widgeta dok dodaje nekoliko specifičnih za označavanje slika.

### Potrebna konfiguracija

#### tenantId

Potrebno je vaše FastComments Tenant ID. Možete ga pronaći u vašem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Specifične opcije za Image Chat

#### urlId

Po zadanim postavkama, Image Chat generira jedinstveni identifikator za svaki razgovor na temelju URL stranice, izvora slike i X/Y koordinata. To možete nadjačati vlastitim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ovo je korisno kada se vaša struktura URL-a može promijeniti, ali želite zadržati iste razgovore, ili kada želite dijeliti bilješke između više stranica.

#### chatSquarePercentage

Kontrolira veličinu klikabilnih oznaka za chat kao postotak širine slike. Zadano je 5%, što znači da je svaka oznaka 5% širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Veći, vidljiviji markeri
});
```

Manje vrijednosti stvaraju manje nametljive markere koji bolje rade za detaljne slike. Veće vrijednosti čine markere lakšima za vidjeti i kliknuti na zauzetim slikama ili za korisnike na mobilnim uređajima.

#### hasDarkBackground

Omogućite stilizaciju tamnog načina rada kada vaša stranica ima tamnu pozadinu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcija povratnog poziva koja se poziva kad god se broj komentara promijeni. Ovo je korisno za ažuriranje elemenata sučelja poput bedževa ili naslova stranice.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Naslijeđene opcije konfiguracije

Budući da Image Chat proširuje standardni widget za komentare, možete koristiti bilo koju opciju konfiguracije iz osnovnog FastComments widgeta. Evo nekih često korištenih opcija:

#### locale

Postavite jezik za korisničko sučelje widgeta. FastComments podržava desetke jezika.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Španjolski
});
```

#### readonly

Učinite sve razgovore samo za čitanje. Korisnici mogu pregledavati postojeće markere i rasprave, ali ne mogu stvarati nove niti odgovarati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrirajte se s vašim sustavom autentikacije koristeći Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Konfiguracija SSO-a
    }
});
```

Pogledajte SSO dokumentaciju za potpune detalje o opcijama autentikacije.

#### maxReplyDepth

Kontrolirajte koliko nivoa duboko odgovori mogu ići. Zadano, Image Chat postavlja ovo na 0, što znači da su svi komentari ravni (nema ugniježđenih odgovora). To možete promijeniti ako želite threaded razgovore.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dozvoli 3 nivoa ugniježđivanja
});
```

### Interna konfiguracija

Ove opcije automatski postavlja Image Chat i ne bi se trebale nadjačavati:

`productId` se automatski postavlja na `2` za Image Chat. Ekstenzija `floating-chat` se automatski učitava kako bi pružila funkcionalnost chat prozora. Widget automatski detektira mobilne uređaje (zasloni uži od 768px) i odgovarajuće prilagođava korisničko sučelje s chat prozorima preko cijelog zaslona.

### Fleksibilnost ciljnog elementa

Prvi parametar za `FastCommentsImageChat` može biti ili `<img>` element izravno ili kontejner element s unutarnjom slikom:

```javascript
// Izravni element slike
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontejner s unutarnjom slikom
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget će automatski pronaći sliku ako proslijedite kontejner element.

### Kompletan primjer

Evo primjera koji prikazuje nekoliko konfiguracijskih opcija zajedno:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Vaša SSO konfiguracija
    },
    maxReplyDepth: 1
});
```

Za potpuni popis svih dostupnih opcija konfiguracije naslijeđenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju o konfiguraciji.