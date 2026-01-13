### Pregled

FastComments Image Chat proširuje standardni FastComments widget za komentare, tako da nasleđuje sve opcije konfiguracije iz osnovnog widgeta dok dodaje nekoliko opcija specifičnih za anotacije slika.

### Obavezna konfiguracija

#### tenantId

Vaš FastComments Tenant ID je obavezan. Možete ga pronaći u vašem [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Opcije specifične za Image Chat

#### urlId

Po defaultu, Image Chat generiše jedinstveni identifikator za svaki razgovor zasnovan na URL-u stranice, izvoru slike i X/Y koordinatama. Možete to prebrisati sa prilagođenim `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ovo je korisno kada se struktura vaših URL-ova može promeniti ali želite zadržati iste razgovore, ili kada želite deliti anotacije preko više stranica.

#### chatSquarePercentage

Kontroliše veličinu klikabilnih markera za chat kao procenat širine slike. Podrazumevano je 5%, što znači da je svaki marker 5% širine slike.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Veći, vidljiviji markeri
});
```

Manje vrednosti stvaraju manje intruzivne markere koji bolje rade za detaljne slike. Veće vrednosti čine markere lakšim za videti i klik na zauzetim slikama ili za korisnike na mobilnim uređajima.

#### hasDarkBackground

Omogućite stilizaciju za tamni režim kada vaša stranica ima tamnu pozadinu.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Funkcija povratnog poziva koja se izvršava kad god se broj komentara promeni. Ovo je korisno za ažuriranje UI elemenata kao što su značke ili naslovi stranica.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Nasleđene opcije konfiguracije

Pošto Image Chat proširuje standardni widget za komentare, možete koristiti bilo koju opciju konfiguracije iz osnovnog FastComments widgeta. Evo nekoliko često korišćenih opcija:

#### locale

Podesite jezik za UI widgeta. FastComments podržava desetine jezika.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

Napravite sve razgovore samo za čitanje. Korisnici mogu pregledati postojeće markere i diskusije, ali ne mogu kreirati nove ili odgovarati.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integracija sa vašim sistemom autentifikacije koristeći Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO konfiguracija
    }
});
```

Pogledajte SSO dokumentaciju za potpune detalje o opcijama autentifikacije.

#### maxReplyDepth

Kontrolišite koliko nivoa duboko odgovori mogu ići. Po defaultu, Image Chat postavlja ovo na 0, što znači da su svi komentari plosnati (nema ugnježdenih odgovora). Možete ovo promeniti ako želite prikaz sa dretvama.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dozvoli 3 nivoa ugnježđavanja
});
```

### Interna konfiguracija

Ove opcije se automatski podešavaju od strane Image Chat-a i ne bi trebalo da ih prepisujete:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Fleksibilnost ciljnog elementa

Prvi parametar za `FastCommentsImageChat` može biti direktno `<img>` element ili element kontejnera koji ima sliku unutra:

```javascript
// Direktan <img> element
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontejner sa slikom unutra
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Widget će automatski pronaći sliku ako prosledite element kontejnera.

### Kompletan primer

Evo primera koji prikazuje više opcija konfiguracije zajedno:

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

Za kompletan spisak svih dostupnih opcija konfiguracije nasleđenih iz osnovnog widgeta, pogledajte glavnu FastComments dokumentaciju o konfiguraciji.