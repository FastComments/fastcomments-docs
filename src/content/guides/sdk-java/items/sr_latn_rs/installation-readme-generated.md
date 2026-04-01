### Maven

Dodajte Repsy repozitorijum u POM vašeg projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Zatim dodajte zavisnosti koje su vam potrebne:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorijum u vaš build.gradle fajl:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula. Generisani API klijent, core Java biblioteka koja sadrži ručno napisane pomoćne funkcije radi olakšavanja rada sa API-jem, i `pubsub` modul koji je biblioteka za prijavljivanje na feedove promena.

- [Dokumentacija API klijenta](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija Core biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zaštićeni API-ji

Za API klijenta postoje dve klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.