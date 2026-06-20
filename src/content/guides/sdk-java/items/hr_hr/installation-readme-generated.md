### Maven

Dodajte Repsy repozitorij u POM vašeg projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Zatim dodajte potrebne ovisnosti:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij u datoteku build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Ova biblioteka sadrži tri modula. Generirani API klijent, core Java biblioteka koja sadrži ručno napisane pomoćne funkcije za lakši rad s API-jem, i modul `pubsub` koji je biblioteka za pretplatu na feedove promjena.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući primjere SSO-a](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozivati izravno iz preglednika/mobilnog uređaja/itd. bez autentifikacije.

`ModerationApi` pokreće nadzornu ploču moderatora. Sadrži metode za moderiranje komentara (listanje, brojanje, pretraživanje, zapisi i izvoz), akcije moderiranja (ukloni/vrati, označi, postavi status za pregled/spam/odobrenje, glasovi i ponovno otvori/zatvori temu), zabrane (zabrana komentiranja, poništi zabranu, sažeci prije zabrane, status i postavke zabrane, i broj zabranjenih korisnika), te značke i povjerenje (dodijeli/ukloni značku, ručne značke, dohvati/postavi faktor povjerenja i interni korisnički profil). Svaka metoda `ModerationApi` prihvaća `sso` parametar tako da se poziv može izvršiti u ime moderatora autentificiranog putem SSO-a.