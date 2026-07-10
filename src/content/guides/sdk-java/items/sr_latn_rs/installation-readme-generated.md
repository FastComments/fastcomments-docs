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
        <version>3.0.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
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
    implementation "com.fastcomments:client:3.0.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Sadržaj biblioteke

Ova biblioteka sadrži tri modula. Generisani API klijent, core Java biblioteka koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, i `pubsub` modul koji je biblioteka za pretplatu na promene feed‑ova.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija core biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub biblioteke](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Javni vs Zaštićeni API‑ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, dok `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja itd. bez autentifikacije.

`ModerationApi` pruža opsežan skup API‑ja za brzu i trenutnu moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili sesijskog kolačića FastComments.com.